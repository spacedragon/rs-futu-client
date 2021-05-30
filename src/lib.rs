#[macro_use]
extern crate num_derive;

pub mod protos;
pub mod codec;
pub mod client;
mod client_actor;
pub mod commands;

use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum ClientError {
    /// Receiving message during reconnecting
    #[display(fmt = "Not connected")]
    NotConnected,
    /// Cancel all waters when connection get dropped
    #[display(fmt = "Disconnected")]
    Disconnected,
}