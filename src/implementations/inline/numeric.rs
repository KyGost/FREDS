use crate::{data::constants::SIZE_INLINE, Data, Error, InlineData, ReferentialData};

impl Data for u64 {
    const KIND: [u8; 1] = [1];
    const IS_INLINE: bool = true;
}
impl InlineData for u64 {
    fn into_inline_data(self) -> Result<[u8; 8], Error> {
        Ok(self.to_be_bytes())
    }
    fn from_inline_data(bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}
impl ReferentialData for u64 {}

impl Data for i64 {
    const KIND: [u8; 1] = [2];
    const IS_INLINE: bool = true;
}
impl InlineData for i64 {
    fn into_inline_data(self) -> Result<[u8; 8], Error> {
        Ok(self.to_be_bytes())
    }
    fn from_inline_data(bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}
impl ReferentialData for i64 {}

impl Data for f64 {
    const KIND: [u8; 1] = [3];
    const IS_INLINE: bool = true;
}
impl InlineData for f64 {
    fn into_inline_data(self) -> Result<[u8; 8], Error> {
        Ok(self.to_be_bytes())
    }
    fn from_inline_data(bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Ok(Self::from_be_bytes(bytes))
    }
}
impl ReferentialData for f64 {}
