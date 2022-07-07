use crate::{data::constants::SIZE_INLINE, impl_inline, Data, InlineData};

impl Data for bool {
    const TYPE: [u8; 1] = [b'b'];
}
impl InlineData for bool {
    fn into_bytes(self) -> [u8; SIZE_INLINE] {
        [self as u8; SIZE_INLINE]
    }
}
impl_inline!(bool);
