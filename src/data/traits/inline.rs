use crate::{data::constants::SIZE_INLINE, Data};
pub trait InlineData: Data {
    fn into_bytes(self) -> [u8; SIZE_INLINE];
}
#[macro_export]
macro_rules! impl_inline {
    ($type: ty) => {
        impl crate::data::ToInline for $type {
            fn into_inline_data(
                self,
                _writer: &mut crate::Writer,
            ) -> [u8; crate::data::constants::SIZE_INLINE] {
                self.into_bytes()
            }
        }
    };
}
pub use impl_inline;
