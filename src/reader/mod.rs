use {
    crate::{data::constants::*, Inline},
    tokio::{
        fs::File,
        io::{AsyncReadExt, BufReader, Error},
    },
};
pub struct Reader {
    reader: BufReader<File>,
    core: Inline,
    kinds: [ReadKind; 2_usize.pow(SIZE_TYPE as u32 * 8)],
}
impl Reader {
    pub async fn new(file: File) -> Result<Self, Error> {
        const READ_KIND: ReadKind = ReadKind {};
        let mut reader = BufReader::new(file);
        let mut core = [0u8; SIZE_TYPE + SIZE_INLINE];
        reader.read(&mut core).await?;
        let core = core.into();
        Ok(Self {
            reader,
            core,
            kinds: [READ_KIND; 2_usize.pow(SIZE_TYPE as u32 * 8)],
        })
    }
}

pub struct ReadKind {}
