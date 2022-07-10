mod inline;
mod referential;
use crate::{
    data::constants::{SIZE_INLINE, SIZE_KIND},
    Inline, Writer,
};
pub use {inline::InlineData, referential::ReferentialData};

#[derive(Debug)]
pub enum Error {
    Unimplemented,
    ExpectedReferentialType,
    ExpectedInlineType,
    ParseError,
}

pub trait Data: InlineData + ReferentialData {
    const KIND: [u8; SIZE_KIND];
    const IS_INLINE: bool = true;
}
impl<T: Data> DataExt for T {}
pub trait DataExt: Data {
    #[cfg(feature = "write")]
    fn into_ref(self, writer: &mut Writer) -> Result<[u8; SIZE_INLINE], Error> {
        Ok(writer.append(self)?.to_be_bytes())
    }
    #[cfg(feature = "write")]
    fn into_inline(self, writer: &mut Writer) -> Result<Inline, Error> {
        let data = if Self::IS_INLINE {
            self.into_inline_data()
        } else {
            self.into_ref(writer)
        }?;
        Ok(Inline {
            data,
            kind: Self::KIND,
        })
    }
    #[cfg(feature = "read")]
    fn from_inline(_inline: Inline) -> Result<Self, Error> {
        /*let data = if Self::IS_INLINE {
            Self::from_inline(inline)
        } else {
            Self::from_ref(inline)
        }*/
        unimplemented!()
    }
}
