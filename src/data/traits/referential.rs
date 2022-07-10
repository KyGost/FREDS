use crate::{Error};
pub trait ReferentialData: Sized {
    #[cfg(feature = "write")]
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        Err(Error::Unimplemented)
    }
    #[cfg(feature = "read")]
    fn from_bytes(_bytes: Vec<u8>) -> Result<Self, Error> {
        Err(Error::Unimplemented)
    }
}
