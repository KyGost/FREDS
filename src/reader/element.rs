use {
    crate::{Error, Inline, Reader, Value},
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
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
        reader: &mut BufReader<File>,
        inline: Inline,
    ) -> Result<&Value, Error> {
        match self {
            Self::Value(value) => Ok(value),
            Self::Size { start, size } => {
                reader.seek(SeekFrom::Start(*start)).await.unwrap();
                let mut bytes = vec![0_u8; *size as usize];
                reader.read(&mut bytes).await.unwrap();
                let value = Value::from_bytes(reader, inline.kind[0], bytes)?;
                *self = Self::Value(value);
                if let Self::Value(value) = self {
                    Ok(value)
                } else {
                    unreachable!()
                }
            }
            Self::Unknown => unimplemented!(),
        }
    }
}
