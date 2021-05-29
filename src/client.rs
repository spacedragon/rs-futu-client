
use actix::prelude::*;


use tokio::time::{Duration};
use crate::protos;


use bytes::BytesMut;
use anyhow::Result;
use crate::client_actor::{ClientActor, Command};



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
}

impl Client {
    pub async fn connect(addr: &str) -> Self {
        let addr = ClientActor::start(addr);
        let mut c = Client {
            addr
        };
        let client_id = uuid::Uuid::new_v4().to_string();
        let init = protos::init_connect::Request {
            c2s: protos::init_connect::C2s {
                client_id,
                recv_notify: None,
                programming_language: Some("rust".to_string()),
                client_ver: 001,
                push_proto_fmt: Some(0),
                packet_enc_algo: None
            }
        };
        let resp: Result<Option<protos::init_connect::Response>> = c.send_request(1001, &init).await;
        if let Ok(Some(resp)) = resp {
            if let Some(s2c) = resp.s2c {
                let keep_alive = Command::KeepAlive(Duration::from_secs(s2c.keep_alive_interval as u64));
                c.addr.send(keep_alive).await;
            }
        }
        c
    }

    pub async fn send_request<S: prost::Message, R: prost::Message + Default>(&mut self, proto_id: u32, msg: &S) -> Result<Option<R>> {
        let mut body = BytesMut::with_capacity(msg.encoded_len());
        msg.encode(&mut body)?;
        let resp = self.addr.send(Command::Request(proto_id, body)).await??;
        if let Some(resp) = resp {
            let resp = R::decode(resp.body)?;
            Ok(Some(resp))
        } else {
            Ok(None)
        }
    }
}

pub enum ClientCommand {
    Init
}

impl Message for ClientCommand {
    type Result = ();
}

#[test]
fn test() {
    System::new().block_on(async {
        let client = Client::connect("127.0.0.1:11111").await;
        sleep(Duration::from_secs(10)).await;
    });
}
