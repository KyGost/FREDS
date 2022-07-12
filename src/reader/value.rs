use crate::{Error, Reader};

pub trait Value: Sized {
    fn from_bytes(reader: &mut Reader<Self>, kind: u8, bytes: Vec<u8>) -> Result<Self, Error>;
}
