use crate::data::constants::*;
#[derive(Clone, Copy, Hash, Eq, PartialEq, Default)]
pub struct Inline {
    pub kind: [u8; SIZE_TYPE],
    pub data: [u8; SIZE_INLINE],
}
impl Into<Vec<u8>> for Inline {
    fn into(self) -> Vec<u8> {
        self.kind.into_iter().chain(self.data).collect() // Can this be done better?
    }
}
impl From<[u8; SIZE_TYPE + SIZE_INLINE]> for Inline {
    fn from(input: [u8; SIZE_TYPE + SIZE_INLINE]) -> Self {
        Self {
            kind: input[..SIZE_TYPE].try_into().unwrap(),
            data: input[SIZE_TYPE..].try_into().unwrap(),
        }
    }
}
