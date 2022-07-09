use crate::{data::constants::SIZE_INLINE, Data, Error, InlineData, ReferentialData};

impl Data for bool {
    const KIND: [u8; 1] = [b'b'];
    const IS_INLINE: bool = true;
}
impl InlineData for bool {
    fn into_inline_data(self) -> Result<[u8; SIZE_INLINE], Error> {
        Ok([self as u8; SIZE_INLINE])
    }
}
impl ReferentialData for bool {}
