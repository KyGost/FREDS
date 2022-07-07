use crate::{impl_referential, Data, Inline, ReferentialData, Writer};

pub struct Array {
    pub data: Vec<Inline>,
}
impl Array {
    pub fn push<Input: Data>(&mut self, writer: &mut Writer, input: Input) {
        self.data.push(writer.into_inline(input));
    }
    pub fn from<Input: Data>(writer: &mut Writer, input: Vec<Input>) -> Self {
        let data = input
            .into_iter()
            .map(|input| writer.into_inline(input))
            .collect();
        Self { data }
    }
}

impl Data for Array {
    const TYPE: [u8; 1] = [b'a'];
}
impl ReferentialData for Array {
    fn to_bytes(self) -> Vec<u8> {
        self.data
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
    fn from_bytes(_bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}
impl_referential!(Array);
