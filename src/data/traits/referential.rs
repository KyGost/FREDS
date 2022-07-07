use crate::Data;
pub trait ReferentialData: Data {
    fn to_bytes(self) -> Vec<u8>;
    fn from_bytes(bytes: Vec<u8>) -> Self;
}
#[macro_export]
macro_rules! impl_referential {
    ($type: ty) => {
        impl crate::data::ToInline for $type {
            fn into_inline_data(
                self,
                writer: &mut crate::Writer,
            ) -> [u8; crate::data::constants::SIZE_INLINE] {
                writer.append(self).to_be_bytes()
            }
        }
    };
}
pub use impl_referential;
