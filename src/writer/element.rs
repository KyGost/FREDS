pub struct Element {
    pub data: Vec<u8>,
}
impl Element {
    pub fn into_bytes(self) -> Vec<u8> {
        let size = self.data.len();
        let size_bytes: [u8; 8] = size.to_be_bytes();
        [size_bytes.as_slice(), &self.data].concat()
    }
}
