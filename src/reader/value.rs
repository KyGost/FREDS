use crate::{Error, Inline, Reader};

pub trait Value: Sized {
    fn from_bytes(kind: u8, bytes: Vec<u8>) -> Result<Self, Error>;
}
