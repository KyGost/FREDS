use crate::{
    data::constants::{SIZE_INLINE, SIZE_TYPE},
    impl_inline, Data, InlineData,
};

pub struct Null;

impl Data for Null {
    const TYPE: [u8; SIZE_TYPE] = [0; SIZE_TYPE];
}
impl InlineData for Null {
    fn into_bytes(self) -> [u8; SIZE_INLINE] {
        [0; SIZE_INLINE]
    }
}
impl_inline!(Null);
