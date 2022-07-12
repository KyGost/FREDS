use {
    crate::{Error, Inline, Reader, Value},
    tokio::io::SeekFrom,
};

#[derive(Debug)]
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
impl<Value: crate::Value> Element<Value> {
    pub async fn get(
        &mut self,
        reader: &mut Reader<Value>,
        inline: Inline,
    ) -> Result<Value, Error> {
        match self {
            Self::Value(value) => Ok(value),
            Self::Size { start, size } => {
                reader.reader.seek(SeekFrom::Start(start)).await.unwrap();
                let mut bytes = vec![0_u8; size];
                reader.reader.read(&mut bytes).await.unwrap();
                Value::from_bytes(self.kind[0], bytes)
            }
            Self::Unknown => unimplemented!(),
        }
    }
}
