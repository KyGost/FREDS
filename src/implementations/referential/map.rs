use {
    crate::{Data, Error, Inline, InlineData, ReferentialData},
    std::collections::HashMap,
};

pub struct Map {
    pub data: HashMap<Inline, Inline>,
}

impl Data for Map {
    const KIND: [u8; 1] = [b'm'];
    const IS_INLINE: bool = false;
}
impl InlineData for Map {}
impl ReferentialData for Map {
    fn into_bytes(self) -> Result<Vec<u8>, Error> {
        Ok(self
            .data
            .into_iter()
            .map(|(key, value)| {
                let key: Vec<u8> = key.into();
                let value: Vec<u8> = value.into();
                key.into_iter().chain(value).collect()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat())
    }
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        use crate::data::constants::*;
        let data: HashMap<Inline, Inline> = bytes
            .chunks_exact((SIZE_KIND + SIZE_INLINE) * 2)
            .map(|d| {
                let (key, value) = d.split_at(SIZE_KIND + SIZE_INLINE);
                let key: [u8; 9] = key.try_into().unwrap();
                let value: [u8; 9] = value.try_into().unwrap();
                let key: Inline = key.into();
                let value: Inline = value.into();
                (key, value)
            })
            .collect();
        Ok(Map { data })
    }
}
