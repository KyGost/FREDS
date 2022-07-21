mod element;
mod kind;
use {
    crate::{
        data::constants::{SIZE_KIND, SIZE_SIZE},
        Data, Error, Inline, Null,
    },
    element::Element,
    kind::Kind,
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
};
pub struct DataManager {
    pub data: [Option<Kind>; 2_usize.pow(SIZE_KIND as u32 * 8)],
    reader: Option<BufReader<File>>,
    pub core: Option<Inline>,
}
impl DataManager {
    fn get_inline_at(&self, position: usize) -> Result<Inline, Error> {

    }
}
