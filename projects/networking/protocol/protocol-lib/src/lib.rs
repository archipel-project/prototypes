mod packet_frame;
mod packets;
mod types;

pub use packet_frame::PacketFrame;

#[doc(hidden)]
pub mod __private {
    pub use crate::packets::types::VarInt;
    pub use crate::types::codec::{Decode, Encode};
    pub use anyhow::{anyhow, bail, ensure, Context, Result};
}

// This allow us to use our own proc macros internally.
extern crate self as slp_lib;
