use crate::{data::constants::SIZE_TYPE, writer::Element, Error};
#[derive(Default)]
pub struct Kind {
    pub data: Vec<Element>,
}
impl Kind {
    pub fn into_bytes(self, kind: [u8; SIZE_TYPE * 8]) -> Vec<u8> {
        let size = self.data.len();
        if size > 0 {
            let data: Vec<u8> = self.data.into_iter().flat_map(|e| e.into_bytes()).collect();
            size.to_be_bytes()
                .into_iter()
                .chain(kind)
                .chain(data)
                .collect()
        } else {
            Vec::new()
        }
    }
    pub fn append(&mut self, data: Vec<u8>) -> Result<usize, Error> {
        self.data.push(Element { data });
        Ok(self.data.len() - 1)
    }
}