use actix_codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};
use serde::{Serialize, Deserialize};
use bincode::Options;
use sha1::{Sha1, Digest};
use anyhow::Result;

pub struct ClientCodec;

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    pub proto_id: u32,
    pub serial_no: u32,
    pub body: BytesMut
}

pub struct ErrPayload {
    pub proto_id: u32,
    pub serial_no: u32,
}

impl Decoder for ClientCodec {
    type Item = Payload;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len()< HEADER_LEN {
            return Ok(None)
        }
        let mut header_buf = src.split_to(HEADER_LEN);
        let header = Header::deserialize(&mut header_buf)?;
        let body_len = header.n_body_len as usize;
        if src.len() < body_len {
            return Ok(None)
        }
        let body_buf = src.split_to(body_len);
        Ok(Some(Payload{
            proto_id: header.n_proto_id,
            serial_no: header.n_serial_no,
            body: body_buf
        }))
    }
}

impl Encoder<Payload> for ClientCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, req: Payload, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let body_len = req.body.len();
        let mut hasher = Sha1::new();
        hasher.update(&req.body);
        let body_sha = hasher.finalize();
        let header = Header::new(req.proto_id, req.serial_no, body_len as u32, body_sha.into());
        header.serialize(dst)?;
        dst.extend_from_slice(&req.body);
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