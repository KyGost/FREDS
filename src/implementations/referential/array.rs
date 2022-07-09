use crate::{Data, DataExt, Error, Inline, InlineData, ReferentialData, Writer};

pub struct Array {
    pub data: Vec<Inline>,
}
impl Array {
    pub fn push<Input: Data>(&mut self, writer: &mut Writer, input: Input) -> Result<(), Error> {
        Ok(self.data.push(input.into_inline(writer)?))
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
}
impl InlineData for Array {}
