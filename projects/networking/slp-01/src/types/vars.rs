use {
    crate::*,
    byteorder::{ReadBytesExt, WriteBytesExt},
    funty::Integral,
    std::io::{Read, Write},
};

#[derive(Debug, Copy, Clone)]
pub struct Var<I: Integral>(pub I);

pub type VarLong = Var<i64>;
pub type VarInt = Var<i32>;

const SEGSZ: usize = 7;

const CONTINUE_BIT: u8 = 1 << SEGSZ;
const SEGMENT_MASK: u8 = CONTINUE_BIT - 1;

impl<I> Codec for Var<I>
where
    I: Integral + From<u8>,
{
    fn encode_to(self, mut w: impl Write) -> Result<(), EncodeError> {
        let mut n = self.0.as_u64();

        while n != 0 {
            let b = n as u8;

            w.by_ref()
                .write_u8(b & SEGMENT_MASK | b & CONTINUE_BIT)
                .map_err(|_| EncodeError)?;

            n >>= SEGSZ;
        }

        Ok(())
    }

    fn decode_from(mut r: impl Read) -> Result<Self, DecodeError> {
        let mut val = I::ZERO;

        for i in 0..Self(!I::ZERO).size() {
            let byte = r.by_ref().read_u8().map_err(|_| DecodeError::Incomplete)?;

            val |= I::from(byte & SEGMENT_MASK) << (i * SEGSZ);

            if byte & CONTINUE_BIT == 0 {
                return Ok(Self(val));
            }
        }

        Err(DecodeError::TooLong)
    }
}

impl<I: Integral> Sizeable for Var<I> {
    fn size(&self) -> usize {
        let n = I::BITS - self.0.leading_zeros();
        (n * 9 / 64 + 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_to_i32() {
        let v = VarInt::decode_from([0xff, 0xff, 0xff, 0xff, 0x0f].as_slice()).unwrap();
        assert_eq!(v.0, -1);
    }

    #[test]
    #[should_panic(expected = "TooLong")]
    fn invalid_varint() {
        // 6 bytes
        VarInt::decode_from([0xff, 0xff, 0xff, 0xff, 0xff, 0xff].as_slice()).unwrap();
    }

    #[test]
    fn varlong_to_i32() {
        let v = VarLong::decode_from(
            [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01].as_slice(),
        )
        .unwrap();

        assert_eq!(v.0, -1);
    }

    #[test]
    fn varlong_size_approx() {
        for i in 0..64 {
            let v = Var(1u64 << i);
            let len = (i + 1) / SEGSZ + ((i + 1) % SEGSZ != 0) as usize; // real length formula
            assert_eq!(v.size(), len, "approximation is invalid");
        }
    }

    #[test]
    #[should_panic(expected = "TooLong")]
    fn invalid_varlong() {
        // 11 bytes
        VarInt::decode_from(
            [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ]
            .as_slice(),
        )
        .unwrap();
    }
}
