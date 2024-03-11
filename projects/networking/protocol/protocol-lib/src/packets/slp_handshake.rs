use crate::packets::types::PacketState;
use protocol_lib_derive::{Decode, Encode, Packet};

use crate::types::str::Bounded;

use super::types::VarInt;

#[derive(Debug, Encode, Decode)]
pub enum HandshakeNextState {
    #[packet(value = 1)]
    Status,
    #[packet(value = 2)]
    Login,
}

#[derive(Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Handshaking)]
pub struct SlpHandshakeC2S<'a> {
    pub protocol_version: VarInt,
    pub server_address: Bounded<&'a str, 255>,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}
