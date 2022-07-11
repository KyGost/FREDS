use crate::{
    data::constants::{SIZE_INLINE, SIZE_KIND},
    Data, Error, InlineData, ReferentialData,
};

pub struct Null;

impl Data for Null {
    const KIND: [u8; SIZE_KIND] = [0; SIZE_KIND];
    const IS_INLINE: bool = true;
}
impl InlineData for Null {
    fn into_inline_data(self) -> Result<[u8; SIZE_INLINE], Error> {
        Ok([0; SIZE_INLINE])
    }
    fn from_inline_data(_bytes: [u8; SIZE_INLINE]) -> Result<Self, Error> {
        Ok(Null)
    }
}
impl ReferentialData for Null {}
