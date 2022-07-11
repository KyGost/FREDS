use crate::{data::constants::SIZE_KIND, writer::Element, Error};
#[derive(Default)]
pub struct Kind {
    pub data: Vec<Element>,
}
impl Kind {
    pub fn into_bytes(self, kind: [u8; SIZE_KIND]) -> Vec<u8> {
        if self.data.len() > 0 {
            let data_bytes: Vec<u8> = self.data.into_iter().flat_map(|e| e.into_bytes()).collect();
            let size = data_bytes.len();
            if size == 0 {
                return Vec::new();
            }
            let size_bytes: [u8; 8] = size.to_be_bytes();
            [
                size_bytes.as_slice(),
                kind.as_slice(),
                data_bytes.as_slice(),
            ]
            .concat()
        } else {
            Vec::new()
        }
    }
    pub fn append(&mut self, data: Vec<u8>) -> Result<u64, Error> {
        self.data.push(Element { data });
        Ok(self.data.len() as u64 - 1)
    }
}
