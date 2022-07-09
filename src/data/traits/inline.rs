use crate::{data::constants::SIZE_INLINE, Error};
pub trait InlineData: Sized {
    #[cfg(feature = "write")]
    fn into_inline_data(self) -> Result<[u8; SIZE_INLINE], Error> {
        Err(Error::Unimplemented)
    }
    #[cfg(feature = "read")]
    fn from_inline_data(_bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Err(Error::Unimplemented)
    }
}
