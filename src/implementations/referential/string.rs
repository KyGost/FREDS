use crate::{Data, Error, InlineData, ReferentialData};

impl Data for String {
    const KIND: [u8; 1] = [b's'];
    const IS_INLINE: bool = false;
}
impl ReferentialData for String {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self.as_bytes().to_vec())
    }
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        Self::from_utf8(bytes).map_err(|_| Error::ParseError)
    }
}
impl InlineData for String {}
