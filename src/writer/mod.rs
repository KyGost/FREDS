mod element;
mod kind;
use crate::{data::constants::SIZE_KIND, Error, Inline};
pub use {element::Element, kind::Kind};
pub struct Writer {
    data: [Kind; 2_usize.pow(SIZE_KIND as u32 * 8)],
    core: Option<Inline>,
}
impl Default for Writer {
    fn default() -> Self {
        const KIND: Kind = Kind { data: Vec::new() };
        Self {
            data: [KIND; 2_usize.pow(SIZE_KIND as u32 * 8)],
            core: None,
        }
    }
}
impl Writer {
    pub fn set_core(&mut self, core: Inline) {
        self.core = Some(core);
    }
    pub fn append<Data: crate::Data>(&mut self, data: Data) -> Result<u64, Error> {
        let dataset = &mut self.data[Data::KIND[0] as usize]; // Indexing TYPE at [0] defeats the purpose of its type
        dataset.append(data.into_bytes()?)
    }
    pub fn into_bytes(self) -> Vec<u8> {
        let core_bytes: Vec<u8> = self.core.unwrap_or_default().into();
        assert!(core_bytes.len() == 9);
        let data_bytes: Vec<u8> = self
            .data
            .into_iter()
            .enumerate()
            .flat_map(|(kind_ident, kind_data)| kind_data.into_bytes([kind_ident as u8]))
            .collect();
        [core_bytes, data_bytes].concat()
    }
}

pub trait Write {
    fn write(self) -> Vec<u8>;
}
