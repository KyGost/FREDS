use {
    crate::{Error, Inline, Reader, Value},
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
};

#[derive(Debug, Clone)]
pub enum Element<Value: crate::Value> {
    Unknown,
    Size { start: u64, size: u64 },
    Value(Value),
}
impl<Value: crate::Value> Default for Element<Value> {
    fn default() -> Self {
        Self::Unknown
    }
}
