use byteorder::{ReadBytesExt, WriteBytesExt};
use std::{
    io::{Read, Write},
    usize,
};

use crate::{
    __private::{Decode, Encode},
    types::errors::EncodeError,
};

use super::var_numbers::{VarDecodeError, CONTINUE_BIT, SEGMENT_MASK, SEGSZ};

#[derive(Debug, Copy, Clone)]
pub struct VarInt(pub i32);

impl VarInt {
    /// The maximum number of bytes a VarInt could occupy when read from and
    /// written to the Minecraft protocol.
    pub const MAX_SIZE: usize = 5;

    /// Returns the exact number of bytes this varint will write when
    /// [`Encode::encode`] is called, assuming no error occurs.
    pub const fn written_size(&self) -> usize {
        match self.0 {
            0 => 1,
            n => (32 - n.leading_zeros() as usize) / 7 + 1,
        }
    }

    pub fn decode_partial(mut r: impl Read) -> Result<i32, VarDecodeError> {
        let mut val = 0;

        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8().map_err(|_| VarDecodeError::Incomplete)?;
            val |= ((byte & SEGMENT_MASK) as i32) << (i * 7);
            if byte & CONTINUE_BIT == 0 {
                return Ok(val);
            }
        }

        Err(VarDecodeError::TooLarge)
    }
}

impl Encode for VarInt {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let mut n = self.0 as u32;

        while n != 0 {
            let b = n as u8;

            w.by_ref()
                .write_u8(b & SEGMENT_MASK | b & CONTINUE_BIT)
                .map_err(|_| EncodeError)?;

            n >>= SEGSZ;
        }
        Ok(())
    }
}

impl Decode<'_> for VarInt {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8()?;
            val |= ((byte & SEGMENT_MASK) as i32) << (i * 7);
            if byte & CONTINUE_BIT == 0 {
                return Ok(Self(val));
            }
        }

        Err(VarDecodeError::TooLarge.into())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn varint_to_i32() {
        let v = VarInt::decode(&mut [0xff, 0xff, 0xff, 0xff, 0x0f].as_slice()).unwrap();
        assert_eq!(v.0, -1);
    }

    #[test]
    fn invalid_varint() -> Result<()> {
        // 6 bytes
        let result = VarInt::decode(&mut [0xff, 0xff, 0xff, 0xff, 0xff, 0xff].as_slice());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Var is too large".to_string()
        );
        Ok(())
    }
}
