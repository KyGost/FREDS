#[derive(Debug, Clone)]
pub enum Element {
    Unknown,
    Size { start: u64, size: u64 },
    Bytes(Vec<u8>),
}
impl Default for Element {
    fn default() -> Self {
        Self::Unknown
    }
}
