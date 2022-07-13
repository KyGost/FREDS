use crate::{data::constants::SIZE_KIND, Error, Reader};
use async_trait::async_trait;

#[async_trait]
pub trait Value: Sized + Clone {
    async fn from_bytes(
        reader: &mut Reader<Self>,
        kind: [u8; SIZE_KIND],
        bytes: Vec<u8>,
    ) -> Result<Self, Error>;
}
