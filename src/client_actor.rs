use actix::prelude::*;
use actix_rt::net::TcpStream;
use actix_service::boxed::{service, BoxService};
use actix_tls::connect::{default_connector, Connect, ConnectError, Connection};
use backoff::backoff::Backoff;
use backoff::ExponentialBackoff;
use log::{debug, error, log_enabled, info, warn, Level};
use tokio::io::{split, WriteHalf};
use tokio::sync::{watch, oneshot};
use tokio_util::codec::FramedRead;
use crate::codec::{Payload, ClientCodec};
use intmap::IntMap;
use tokio::sync::oneshot::{Sender};
use anyhow::{Result, Error};
use crate::{ClientError, protos};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use bytes::BytesMut;
use crate::commands::{ProtoId, Notify};
use num_traits::*;

type Resp = Option<Payload>;
type Req = Payload;

#[derive(Debug)]
pub enum Command {
    Request(u32, BytesMut),
    KeepAlive(Duration),
    Notify(watch::Sender<Notify>),
}

impl Message for Command {
    type Result = anyhow::Result<Resp>;
}

/// Redis communication actor
pub struct ClientActor {
    addr: String,
    connector: BoxService<Connect<String>, Connection<String, TcpStream>, ConnectError>,
    backoff: ExponentialBackoff,
    cell: Option<actix::io::FramedWrite<Req, WriteHalf<TcpStream>, ClientCodec>>,
    promises: IntMap<Sender<Result<Resp>>>,
    seq_no: AtomicU32,
    watch: Option<watch::Sender<Notify>>,
}

impl ClientActor {
    /// Start new `Supervisor`.
    pub fn start<S: Into<String>>(addr: S) -> Addr<ClientActor> {
        let addr = addr.into();

        let backoff = ExponentialBackoff {
            max_elapsed_time: None,
            ..Default::default()
        };

        Supervisor::start(|_| ClientActor {
            addr,
            connector: service(default_connector()),
            cell: None,
            backoff,
            promises: IntMap::new(),
            seq_no: AtomicU32::new(1),
            watch: None,
        })
    }

    fn hb(&mut self) {
        use prost::Message;

        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let keep_alive = protos::keep_alive::Request {
            c2s: protos::keep_alive::C2s {
                time: since_the_epoch.as_secs() as i64
            }
        };
        let mut body = BytesMut::with_capacity(keep_alive.encoded_len());
        if let Ok(()) = keep_alive.encode(&mut body) {
            let payload = self.create_payload(ProtoId::KeepAlive as u32, body);
            if let Some(ref mut cell) = self.cell {
                debug!("sending heart beat");
                cell.write(payload);
            }
        }
    }

    fn create_payload(&mut self, proto_id: u32, body: BytesMut) -> Payload {
        let serial_no = self.seq_no.fetch_add(1, Ordering::SeqCst);
        let payload = Payload {
            body,
            proto_id,
            serial_no,
        };
        payload
    }
}

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let req = Connect::new(self.addr.to_owned());
        self.connector
            .call(req)
            .into_actor(self)
            .map(|res, act, ctx| match res {
                Ok(conn) => {
                    let stream = conn.into_parts().0;
                    info!("Connected to server: {}", act.addr);

                    let (r, w) = split(stream);

                    // configure write side of the connection
                    let framed = actix::io::FramedWrite::new(w, ClientCodec, ctx);
                    act.cell = Some(framed);

                    // read side of the connection
                    ctx.add_stream(FramedRead::new(r, ClientCodec));

                    act.backoff.reset();
                }
                Err(err) => {
                    error!("Can not connect to server: {}", err);
                    // re-connect with backoff time.
                    // we stop current context, supervisor will restart it.
                    if let Some(timeout) = act.backoff.next_backoff() {
                        ctx.run_later(timeout, |_, ctx| ctx.stop());
                    }
                }
            })
            .wait(ctx);
    }
}

impl Supervised for ClientActor {
    fn restarting(&mut self, _: &mut Self::Context) {
        self.cell.take();
        for tx in self.promises.drain() {
            let _ = tx.1.send(Err(Error::from(ClientError::Disconnected)));
        }
    }
}

impl actix::io::WriteHandler<Error> for ClientActor {
    fn error(&mut self, err: Error, _: &mut Self::Context) -> Running {
        warn!("connection dropped: {} error: {}", self.addr, err);
        Running::Stop
    }
}

impl StreamHandler<Result<Payload, Error>> for ClientActor {
    fn handle(&mut self, msg: Result<Payload, Error>, ctx: &mut Self::Context) {
        match msg {
            Err(e) => {
                // if let Some(tx) = self.promises.get_mut() {
                //     let _ = tx.send(Err(e.into()));
                // }
                error!("stream error {}", e);
                ctx.stop();
            }
            Ok(val) => {
                if let Some(tx) = self.promises.remove(val.serial_no as u64) {
                    let _ = tx.send(Ok(Some(val)));
                } else {
                    match ProtoId::from_u32(val.proto_id) {
                        Some(ProtoId::KeepAlive) => {
                            if log_enabled!(Level::Debug) {
                                use prost::Message;
                                if let Ok(resp) = protos::keep_alive::Response::decode(val.body) {
                                    if let Some(s2c) = resp.s2c {
                                        debug!("heart beat resp, service time is {}", s2c.time)
                                    }
                                }
                            }
                        }
                        Some(ProtoId::Notify) => {
                            if let Some(sender) = &self.watch {
                                use prost::Message;
                                if let Ok(notify) =  Notify::decode(val.body) {
                                    if let Err(e) = sender.send(notify) {
                                        error!("send notify failed. {}", e);
                                    }
                                } else {
                                    error!("invalid notify returned from server")
                                }
                            } else {
                                debug!("notify ignored, no receiver.")
                            };
                        }
                        _ => {
                            if log_enabled!(Level::Warn) {
                                warn!("unhandled server response proto:id {}, seq_no: {}", val.proto_id, val.serial_no);
                            }
                        }
                    };
                }
            }
        }
    }
}

impl Handler<Command> for ClientActor {
    type Result = ResponseFuture<Result<Resp>>;

    fn handle(&mut self, msg: Command, ctx: &mut Self::Context) -> Self::Result {
        if self.cell.is_none() {
            return Box::pin(async { Err(anyhow::Error::from(ClientError::NotConnected)) });
        }
        match msg {
            Command::Request(proto_id, body) => {
                let (tx, rx) = oneshot::channel();
                let payload = self.create_payload(proto_id, body);
                self.promises.insert(payload.serial_no as u64, tx);
                if let Some(ref mut cell) = self.cell {
                    cell.write(payload);
                }
                Box::pin(async move {
                    rx.await.map_err(|_| ClientError::Disconnected)?
                })
            }
            Command::KeepAlive(interval) => {
                info!("send heart beat every {} secs", interval.as_secs());
                ctx.run_interval(interval, move |act, _| {
                    act.hb()
                });
                Box::pin(async { Ok(None) })
            }
            Command::Notify(sender) => {
                self.watch = Some(sender);
                Box::pin(async { Ok(None) })
            }
        }
    }
}

mod test {
    use anyhow::Result;
    use actix_rt::System;
    use actix_rt::time::sleep;
    use std::time::Duration;

    #[test]
    pub fn test_watch() -> Result<()> {
        System::new().block_on(async {
            use tokio::sync::watch;

            let (tx, mut rx) = watch::channel("hello");

            tokio::spawn(async move {
                while rx.changed().await.is_ok() {
                    println!("received = {:?}", *rx.borrow());
                }
            });

            tx.send("world")?;
            sleep(Duration::from_secs(1)).await;
            Ok(())
        })
    }
}
