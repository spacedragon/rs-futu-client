use actix::prelude::*;
use std::str::FromStr;
use std::{io, net, thread};
use tokio::io::{split, WriteHalf};
use tokio::net::TcpStream;
use tokio_util::codec::FramedRead;
use crate::codec;
use crate::codec::ClientRequest;
use crate::protos::init_connect;
use tokio::time::{sleep, Duration};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Client {
    addr: Addr<FutuTcpClient>,
}

struct FutuTcpClient {
    framed: actix::io::FramedWrite<
        codec::ClientRequest,
        WriteHalf<TcpStream>,
        codec::ClientCodec,
    >,
    client_id: String,
    seq_no: AtomicU32
}


impl Client {
    pub async fn connect(addr: &str) -> Self {
        let addr = net::SocketAddr::from_str(addr).unwrap();
        let stream = TcpStream::connect(&addr).await.unwrap();
        let client_id = uuid::Uuid::new_v4().to_string();

        let addr = FutuTcpClient::create(|ctx| {
            let (r, w) = split(stream);
            FutuTcpClient::add_stream(FramedRead::new(r, codec::ClientCodec), ctx);
            FutuTcpClient {
                framed: actix::io::FramedWrite::new(w, codec::ClientCodec, ctx),
                client_id,
                seq_no:AtomicU32::new(1)
            }
        });

        Client {
            addr
        }

    }
}

impl Actor for FutuTcpClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        let payload = init_connect::Request {
            c2s: init_connect::C2s {
                programming_language: Some("rust".to_string()),
                client_id: self.client_id.to_string(),
                client_ver: 001,
                push_proto_fmt: Some(0),
                recv_notify: Some(true),
                packet_enc_algo: None,
            }
        };
        let seq_no = self.seq_no.fetch_add(1, Ordering::SeqCst);
        let init_connect = ClientRequest::InitConnect(seq_no, payload);
        self.framed.write(init_connect)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        //System::current().stop();
    }
}

impl actix::io::WriteHandler<io::Error> for FutuTcpClient {}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand {}

impl Handler<ClientCommand> for FutuTcpClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _: &mut Context<Self>) {
        todo!()
    }
}

/// Server communication

impl StreamHandler<Result<codec::ClientResponse, io::Error>> for FutuTcpClient {
    fn handle(
        &mut self,
        msg: Result<codec::ClientResponse, io::Error>,
        ctx: &mut Context<Self>,
    ) {
        todo!()
        // match msg {
        //     Ok(codec::Response::Message(ref msg)) => {
        //         println!("message: {}", msg);
        //     }
        //     Ok(codec::Response::Joined(ref msg)) => {
        //         println!("!!! joined: {}", msg);
        //     }
        //     Ok(codec::Response::Rooms(rooms)) => {
        //         println!("\n!!! Available rooms:");
        //         for room in rooms {
        //             println!("{}", room);
        //         }
        //         println!();
        //     }
        //     _ => ctx.stop(),
        // }
    }
}
 
#[test]
fn test() {
    System::new().block_on(async {
        let client = Client::connect("127.0.0.1:11111").await;
        sleep(Duration::from_secs(100)).await;
    });

}