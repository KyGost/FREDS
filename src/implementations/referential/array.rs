use crate::{Data, DataExt, Error, Inline, InlineData, ReferentialData, Writer};

pub struct Array {
    pub data: Vec<Inline>,
}
impl Array {
    pub fn push<Input: Data>(&mut self, writer: &mut Writer, input: Input) -> Result<(), Error> {
        self.data.push(input.into_inline(writer)?);
        Ok(())
    }
    pub fn from<Input: Data>(writer: &mut Writer, input: Vec<Input>) -> Result<Self, Error> {
        let data = input
            .into_iter()
            .map(|input| input.into_inline(writer))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { data })
    }
}

impl Data for Array {
    const KIND: [u8; 1] = [b'a'];
    const IS_INLINE: bool = false;
}
impl ReferentialData for Array {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self
            .data
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Vec<u8>>>()
            .concat())
    }
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        use crate::data::constants::*;
        let data: Vec<Inline> = bytes
            .chunks_exact(SIZE_KIND + SIZE_INLINE)
            .map(|d| {
                let d: [u8; 9] = d.try_into().unwrap();
                d
            })
            .map(Inline::from)
            .collect();
        Ok(Array { data })
    }
}
impl InlineData for Array {}
