use actix_codec::{Decoder, Encoder};
use std::io::{self, Read};
use bytes::{BytesMut, BufMut, Buf};
use serde::{Serialize, Deserialize};
use bincode::Options;
use ::prost::Message;
use sha1::{Sha1, Digest};
use crate::protos;

use std::io::ErrorKind;
use num_traits::FromPrimitive;




pub enum ClientRequest {
    InitConnect(u32, protos::init_connect::Request),
    GetGlobalState(u32, protos::get_global_state::Request)
}

#[derive(FromPrimitive)]
pub enum Command {
    InitConnect = 1001,
    GetGlobalState = 1002
}

pub enum ClientResponse {
    InitConnect(u32, protos::init_connect::Response),
    GetGlobalState(u32, protos::get_global_state::Response)
}

pub struct ClientCodec;


impl Decoder for ClientCodec {
    type Item = ClientResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let header = Header::deserialize(src).expect("parse header failed");
        let body_len = header.n_body_len as usize;
        let mut body_buf = &src[HEADER_LEN..(HEADER_LEN + body_len)];

        match FromPrimitive::from_u32(header.n_proto_id) {
            Some(Command::InitConnect) => {
                let msg = protos::init_connect::Response::decode(body_buf)?;
                Ok(Some(ClientResponse::InitConnect(header.n_serial_no, msg)))
            },
            Some(Command::GetGlobalState) => {
                let msg = protos::get_global_state::Response::decode(body_buf)?;
                Ok(Some(ClientResponse::GetGlobalState(header.n_serial_no, msg)))
            },
            _ => {
                println!("Unhandled command {}", header.n_proto_id);
                return Ok(None)
            }
        }
    }
}

impl Encoder<ClientRequest> for ClientCodec {
    type Error = io::Error;

    fn encode(
        &mut self,
        msg: ClientRequest,
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let (cmd, seq_no, message) = match msg {
            ClientRequest::InitConnect(seq_no, body) => (Command::InitConnect, seq_no, body),
            _ => return Result::Err(io::Error::new(ErrorKind::Other, "Unsupported request type"))
        };
        let body_len = message.encoded_len();
        let mut body_buf: Vec<u8> = Vec::with_capacity(body_len);
        message.encode(&mut body_buf)?;

        let mut hasher = Sha1::new();
        hasher.update(&body_buf);
        let body_sha = hasher.finalize();

        let header = Header::new(cmd as u32, seq_no, body_len as u32, <[u8; 20]>::from(body_sha));
        header.serialize(dst).expect("write header failed.");
        dst.put_slice(&body_buf);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    sz_header_flag: [u8; 2],
    n_proto_id: u32,
    n_proto_fmt_type: u8,
    n_proto_ver: u8,
    n_serial_no: u32,
    n_body_len: u32,
    arr_body_sha1: [u8; 20],
    arr_reserved: [u8; 8],
}

impl Header {
    pub fn new(proto_id: u32, serial_no: u32, body_len: u32, body_sha1: [u8; 20]) -> Self {
        Self {
            sz_header_flag: [b'F', b'T'],
            n_proto_id: proto_id,
            n_proto_fmt_type: 0,
            n_proto_ver: 0,
            n_serial_no: serial_no,
            n_body_len: body_len,
            arr_body_sha1: body_sha1,
            arr_reserved: [0; 8],
        }
    }

    pub fn serialize(self, dst: &mut BytesMut) -> bincode::Result<()> {
        let options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();
        options.serialize_into(dst.writer(), &self)
    }

    pub fn deserialize(src: &mut BytesMut) -> bincode::Result<Self> {
        let options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();
        options.deserialize_from(&src[..HEADER_LEN])
    }
}

const HEADER_LEN: usize = 44;

#[test]
fn test() {
    let my_options = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes();
    let header = Header::new(1001, 1, 10, [1; 20]);
    assert_eq!(my_options.serialized_size(&header).unwrap(), HEADER_LEN as u64);
    println!("{:?}", my_options.serialize(&header));
}