pub mod types;

use {
    derive_more::Display,
    std::{
        error::Error,
        io::{Read, Write},
        mem
    }
};

pub trait Sizeable: Sized {
    fn size(&self) -> usize;
}

impl<I: funty::Integral> Sizeable for I {
    fn size(&self) -> usize {
        mem::size_of::<I>()
    }
}

pub trait Codec: Sizeable {
    fn encode_to(self, w: impl Write) -> Result<(), EncodeError>;
    fn decode_from(r: impl Read) -> Result<Self, DecodeError>;
}

#[derive(Display, Debug, Copy, Clone)]
pub struct EncodeError;

impl Error for EncodeError {}

#[derive(Display, Debug, Copy, Clone)]
pub enum DecodeError {
    #[display(fmt = "malformed packet")]
    Incomplete,
    #[display(fmt = "item too long")]
    TooLong
}

impl Error for DecodeError {}
