use crate::{Data, DataExt, Error, Inline, InlineData, Ref, ReferentialData, Value, Writer};

#[derive(Clone, Debug)]
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
    pub fn get_direct_ref<Kind: Value>(index: usize) -> Result<Option<Ref<Kind>>, Error> {
        Err(Error::Unimplemented)
    }
}

const ELEMENT_SIZE: usize = crate::data::constants::SIZE_KIND + crate::data::constants::SIZE_INLINE;

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
        let data: Vec<Inline> = bytes
            .chunks_exact(ELEMENT_SIZE)
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

use crate::{constants::SIZE_KIND, Reader};
#[async_trait::async_trait]
impl Value for Array {
    async fn from_bytes(
        reader: &mut Reader<Self>,
        kind: [u8; SIZE_KIND],
        bytes: Vec<u8>,
    ) -> Result<Self, Error> {
        ReferentialData::from_bytes(bytes)
    }
    async fn from_inline(reader: &mut Reader<Self>, inline: Inline) -> Result<Self, Error> {
        InlineData::from_inline_data(inline.data)
    }
}
use crate::{RefRelation, Relation, DM};
impl RefRelation for Ref<Array> {
    fn get_ref<Kind: Value>(
        dm: &DM,
        reference: Ref<Array>,
        relation: Relation,
    ) -> Result<Option<Ref<Kind>>, Error> {
        if reference.inline.kind != Array::KIND {
            Err(Error::InvalidKind)
        } else {
            match relation {
                Relation::Index(index) => reference.get_direct_ref(dm, index),
                _ => Err(Error::Unimplemented),
            }
        }
    }
}
impl Ref<Array> {
    fn get_direct_ref<Kind: Value>(
        self,
        dm: &DM,
        index: usize,
    ) -> Result<Option<Ref<Kind>>, Error> {
        let position = index * ELEMENT_SIZE;
        let kind_store = dm.kind[Array::KIND];
        if position > kind_store.size {
            Ok(None)
        } else {
            dm.get_inline_at(kind_store.start + position)
        }

        unimplemented!();
    }
}
