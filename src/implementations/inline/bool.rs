use crate::{data::constants::SIZE_INLINE, Data, Error, InlineData, ReferentialData};

impl Data for bool {
    const KIND: [u8; 1] = [b'b'];
    const IS_INLINE: bool = true;
}
impl InlineData for bool {
    fn into_inline_data(self) -> Result<[u8; SIZE_INLINE], Error> {
        Ok([self as u8; SIZE_INLINE])
    }
    fn from_inline_data(bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Ok(bytes[0] == 1)
    }
}
impl ReferentialData for bool {}
