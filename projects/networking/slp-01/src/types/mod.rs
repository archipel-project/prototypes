pub mod vars;

use {
    crate::*,
    derive_more::Deref,
    std::borrow::Cow,
    vars::{Var, VarInt}
};

#[derive(Debug, Deref)]
pub struct McStr<'a>(Cow<'a, str>);

impl<'a, S: Into<Cow<'a, str>>> From<S> for McStr<'a> {
    fn from(v: S) -> Self {
        Self(v.into())
    }
}

impl Codec for McStr<'_> {
    fn decode_from(mut r: impl Read) -> Result<Self, DecodeError> {
        let Var(len) = VarInt::decode_from(r.by_ref())?;
        let mut str = String::with_capacity(len as usize);

        r.take(len as u64)
            .read_to_string(&mut str)
            .map_err(|_| DecodeError::Incomplete)?;

        Ok(str.into())
    }

    fn encode_to(self, mut w: impl Write) -> Result<(), EncodeError> {
        Var(self.len()).encode_to(w.by_ref())?;
        w.write_all(self.as_bytes()).map_err(|_| EncodeError)
    }
}

impl Sizeable for McStr<'_> {
    fn size(&self) -> usize {
        let len = Var(self.len());
        len.size() + len.0
    }
}
