use {
    crate::{Data, Error, Inline, ReferentialData, InlineData},
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
}
