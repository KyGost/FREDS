use {
    crate::{impl_referential, Data, Inline, ReferentialData},
    std::collections::HashMap,
};

pub struct Map {
    pub data: HashMap<Inline, Inline>,
}

impl Data for Map {
    const TYPE: [u8; 1] = [b'm'];
}
impl ReferentialData for Map {
    fn to_bytes(self) -> Vec<u8> {
        self.data
            .into_iter()
            .map(|(key, value)| {
                let key: Vec<u8> = key.into();
                let value: Vec<u8> = value.into();
                key.into_iter().chain(value).collect()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
    fn from_bytes(_bytes: Vec<u8>) -> Self {
        unimplemented!()
    }
}
impl_referential!(Map);
