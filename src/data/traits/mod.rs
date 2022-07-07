mod inline;
mod referential;
use crate::{
    data::constants::{SIZE_INLINE, SIZE_TYPE},
    Writer,
};
pub use {inline::InlineData, referential::ReferentialData};

pub trait Data: ToInline {
    const TYPE: [u8; SIZE_TYPE];
}
pub trait ToInline {
    fn into_inline_data(self, writer: &mut Writer) -> [u8; SIZE_INLINE];
}
