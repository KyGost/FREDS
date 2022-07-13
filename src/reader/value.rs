use crate::{data::constants::SIZE_KIND, Error, Reader};

pub trait Value: Sized + Clone {
    fn from_bytes(
        reader: &mut Reader<Self>,
        kind: [u8; SIZE_KIND],
        bytes: Vec<u8>,
    ) -> Result<Self, Error>;
}
