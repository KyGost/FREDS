use {
    crate::{data::constants::*, Inline, FREDS},
    futures::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, BufReader, Cursor},
    std::io::Error,
    std::pin::Pin,
};
impl FREDS {
    pub async fn from_bytes(bytes: Vec<u8>) -> Result<Self, Error> {
        let mut reader = BufReader::new(Cursor::new(bytes));
        let mut freds = Self::default();
        let mut buffer = [0u8; SIZE_TYPE + SIZE_INLINE];
        reader.read(&mut buffer).await?;
        freds.set_core(buffer.into());
        let mut kind = [0u8; SIZE_TYPE];
        let mut size = [0u8; SIZE_SIZE];
        reader.read(&mut kind).await?;
        reader.read(&mut size).await?;
        let size = usize::from_be_bytes(size);
        Pin::new(&mut reader).seek_relative(size as i64).await?;
        Ok(freds)
    }
}
