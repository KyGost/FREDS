use crate::{
    data::constants::{SIZE_INLINE, SIZE_TYPE},
    Data, Error, InlineData, ReferentialData
};

pub struct Null;

impl Data for Null {
    const KIND: [u8; SIZE_TYPE] = [0; SIZE_TYPE];
    const IS_INLINE: bool = true;
}
impl InlineData for Null {
    fn into_inline_data(self) -> Result<[u8; SIZE_INLINE], Error> {
        Ok([0; SIZE_INLINE])
    }
}
impl ReferentialData for Null {}
