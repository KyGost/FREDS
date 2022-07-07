pub struct Element {
    pub data: Vec<u8>,
}
impl Element {
    pub fn into_bytes(self) -> Vec<u8> {
        let size = self.data.len();
        size.to_be_bytes().into_iter().chain(self.data).collect()
    }
}
