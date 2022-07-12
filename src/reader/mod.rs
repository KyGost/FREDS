mod element;
mod kind;
mod value;
pub use value::Value;
use {
    crate::{
        data::constants::{SIZE_KIND, SIZE_SIZE},
        Data, Error, Inline, Null,
    },
    element::Element,
    kind::Kind,
    std::collections::HashMap,
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
};

pub struct Reader<Value: crate::Value> {
    reader: BufReader<File>,
    pub core: Inline,
    data: [Kind<Value>; 2_usize.pow(SIZE_KIND as u32 * 8)],
}
impl<Value: crate::Value> Reader<Value> {
    async fn get(&mut self, inline: Inline) -> Result<Value, Error> {
        let kind = &mut self.data[inline.kind[0]];
        kind.get(self, inline.data)
    }
    /*async fn get_bytes(&mut self, inline: Inline) -> Result<Vec<u8>, Error> {
        let element_number = u64::from_be_bytes(inline.data) as usize;
        println!(
            "[get_bytes] [kind] {} - {}",
            inline.kind[0],
            String::from_utf8(inline.kind.to_vec()).unwrap_or_default()
        );
        let index = self.index.get(&inline.kind).ok_or(Error::InvalidKind)?;
        println!(
            "[get_bytes] [el len] get: {} len: {}",
            element_number,
            index.elements.len()
        );
        index
            .elements
            .get(element_number)
            .ok_or(Error::BadReference)?;
        self.reader
            .seek(SeekFrom::Start(index.start))
            .await
            .unwrap();
        let mut bytes = vec![0; index.size as usize];
        self.reader.read(&mut bytes).await.unwrap();
        Ok(bytes)
    }*/
}
impl<Value: crate::Value> Reader<Value> {
    pub async fn from_file(path: &str) -> Result<Reader, Error> {
        let file = File::open(path).await.unwrap();
        let mut reader = BufReader::new(file);
        let mut core = Inline::BUFFER;
        reader.read(&mut core).await.unwrap();
        let core = core.into();
        let data = [Kind::Unknown; 2_usize.pow(SIZE_KIND as u32 * 8)];
        Ok(Self { reader, core, data })
    }
    async fn fill_kinds(&mut self) {
        loop {
            let mut size = [0; SIZE_SIZE];
            let mut kind = [0; SIZE_KIND];
            self.reader.read(&mut size).await.unwrap();
            self.reader.read(&mut kind).await.unwrap();
            let size = u64::from_be_bytes(size);
            if kind == Null::KIND {
                break;
            } else {
                let start = self.reader.seek(SeekFrom::Current(0)).await.unwrap();
                self.data[kind.into()] = Kind::Size { start, size };
                self.fill_elements(kind.into()).await;
            }
        }
    }
    async fn fill_elements(&mut self, kind: usize) {
        let start = self.data[kind.into()].start;
        let end = start + self.data[kind.into()].size;
        let elements = &mut self.data[kind.into()].elements;
        elements.clear();
        self.reader.seek(SeekFrom::Start(start)).await.unwrap();
        loop {
            let mut size = [0; SIZE_SIZE];
            self.reader.read(&mut size).await.unwrap();
            let size = u64::from_be_bytes(size);
            let start = self.reader.seek(SeekFrom::Current(0)).await.unwrap();
            elements.push(Element::Size { start, size });
            self.reader
                .seek(SeekFrom::Current(size as i64))
                .await
                .unwrap();
            if start + size == end {
                break;
            }
        }
    }
}
