use crate::{data::constants::*, Error};
#[derive(Clone, Copy, Hash, Eq, PartialEq, Default, Debug)]
pub struct Inline {
    pub kind: [u8; SIZE_KIND],
    pub data: [u8; SIZE_INLINE],
}
impl Inline {
    pub const BUFFER: [u8; SIZE_KIND + SIZE_INLINE] = [0; SIZE_KIND + SIZE_INLINE];
}
impl Into<Vec<u8>> for Inline {
    fn into(self) -> Vec<u8> {
        self.kind.into_iter().chain(self.data).collect() // Can this be done better?
    }
}
impl From<[u8; SIZE_KIND + SIZE_INLINE]> for Inline {
    fn from(input: [u8; SIZE_KIND + SIZE_INLINE]) -> Self {
        Self {
            kind: input[..SIZE_KIND].try_into().unwrap(),
            data: input[SIZE_KIND..].try_into().unwrap(),
        }
    }
}
impl TryFrom<Vec<u8>> for Inline {
    type Error = Error;
    fn try_from(input: Vec<u8>) -> Result<Self, Error> {
        Ok(Self {
            kind: input[..SIZE_KIND].try_into().unwrap(),
            data: input[SIZE_KIND..].try_into().unwrap(),
        })
    }
}
