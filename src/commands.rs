#[allow(dead_code)]

use num_derive::{FromPrimitive, ToPrimitive};
use crate::protos::init_connect;
use crate::protos::keep_alive;
use crate::protos::get_global_state;

pub trait ProtoCommand
    where Self: prost::Message + Sized
{
    type REQ: prost::Message;
    type RESP: prost::Message + Default;
    const PROTO_ID: ProtoId;
    fn to_request(self) -> Self::REQ;
}

macro_rules! define_command {
    ($id:ident ,$l:ident) => {
        pub type $id = crate::protos::$l::C2s;
        impl ProtoCommand for crate::protos::$l::C2s {
            type REQ = crate::protos::$l::Request;
            type RESP = crate::protos::$l::Response;
            const PROTO_ID: ProtoId = ProtoId::$id;
            fn to_request(self) -> Self::REQ {
                $l::Request {
                    c2s: self
                }
            }
        }
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum ProtoId {
    InitConnect = 1001,
    GetGlobalState = 1002,
    Notify= 1003,
    KeepAlive = 1004,
}

define_command!(InitConnect, init_connect);
define_command!(KeepAlive, keep_alive);
define_command!(GetGlobalState, get_global_state);






