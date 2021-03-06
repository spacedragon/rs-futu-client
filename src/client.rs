use actix::prelude::*;
use tokio::time::{Duration};
use crate::protos;
use bytes::BytesMut;
use anyhow::Result;
use crate::client_actor::{ClientActor, Command};
use crate::commands::*;
use log::error;
use tokio::sync::{watch, broadcast};
use crate::codec::Payload;

const DEFAULT_SUB_SIZE: usize = 16;

pub enum ClientRequest {
    InitConnect(u32, protos::init_connect::Request),
    GetGlobalState(u32, protos::get_global_state::Request),
}

pub enum ClientResponse {
    InitConnect(u32, protos::init_connect::Response),
    GetGlobalState(u32, protos::get_global_state::Response),
}

pub struct Client {
    addr: Addr<ClientActor>,
    watch: Option<watch::Receiver<Notify>>,
    sub: Option<broadcast::Sender<Payload>>
}

impl Client {
    pub async fn connect(addr: &str) -> Self {
        let addr = ClientActor::start(addr);
        let mut c = Client {
            addr,
            watch: None,
            sub: None
        };
        let client_id = uuid::Uuid::new_v4().to_string();
        let init = protos::init_connect::C2s {
            client_id,
            recv_notify: Some(true),
            programming_language: Some("rust".to_string()),
            client_ver: 001,
            push_proto_fmt: Some(0),
            packet_enc_algo: None
        };

        if let Ok(Some(resp)) = c.send_command(init).await {
            if let Some(s2c) = resp.s2c {
                let keep_alive = Command::KeepAlive(Duration::from_secs(s2c.keep_alive_interval as u64));
                if let Err(e) = c.addr.send(keep_alive).await {
                    error!("send keep alive request failed. {}", e);
                };
            }
        }
        c
    }

    pub async fn send_command<S: ProtoCommand>(&mut self, cmd: S) -> Result<Option<S::RESP>> {
        let msg = cmd.to_request();
        self.send_request(S::PROTO_ID, &msg).await
    }

    async fn send_request<S: prost::Message, R: prost::Message + Default>(&mut self, cmd_id: ProtoId, msg: &S) -> Result<Option<R>> {
        let mut body = BytesMut::with_capacity(msg.encoded_len());
        msg.encode(&mut body)?;
        let resp = self.addr.send(Command::Request(cmd_id as u32, body)).await??;
        if let Some(resp) = resp {
            let resp = R::decode(resp.body)?;
            Ok(Some(resp))
        } else {
            Ok(None)
        }
    }

    pub async fn notification(&mut self) -> watch::Receiver<Notify> {
        if let Some(receiver) = &self.watch {
            receiver.clone()
        } else {
            let (sender,receiver) = watch::channel(Notify::default());
            self.addr.send(Command::Notify(sender)).await
                .expect("set notify failed").expect("set notify failed");
            receiver.clone()
        }
    }

    pub async fn subscribe<S: SubscribeCommand + ProtoCommand>(&mut self, sub: S) -> Result<impl Stream<Item = S::SubResult>> {
        let mut rx = if self.sub.is_none() {
            let (tx, rx) = broadcast::channel(DEFAULT_SUB_SIZE);
            self.sub = Some(tx.clone());
            self.addr.send(Command::Subscribe(tx)).await
                .expect("set subscription failed").expect("set subscription failed");
            rx
        } else {
            self.sub.as_ref().unwrap().subscribe()
        };
        self.send_command(sub).await?;

        let stream = async_stream::stream! {
            let mut received = rx.recv().await;
            while Err(broadcast::error::RecvError::Closed) != received {
                if let Ok(payload) = received {
                    if payload.proto_id == S::SUB_ID as u32 {
                        use prost::Message;
                        if let Ok(result) = S::SubResult::decode(payload.body) {
                            yield result;
                        }
                    }
                }
                received = rx.recv().await
            }
        };
        Ok(stream)
    }
}

pub enum ClientCommand {
    Init
}

impl Message for ClientCommand {
    type Result = ();
}

mod test {
    use actix::System;
    use crate::client::Client;
    use actix::clock::sleep;
    use std::time::Duration;

    #[test]
    fn test() {
        System::new().block_on(async {
            let client = Client::connect("127.0.0.1:11111").await;
            sleep(Duration::from_secs(10)).await;
        });
    }
}
