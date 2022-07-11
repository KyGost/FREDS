use {
    crate::{
        data::constants::{SIZE_KIND, SIZE_SIZE},
        Data, Error, Inline, Null,
    },
    std::collections::HashMap,
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
};

pub struct Reader {
    reader: BufReader<File>,
    pub core: Inline,
    index: HashMap<[u8; SIZE_KIND], KindIndex>,
}
impl Reader {
    pub async fn get_bytes(&mut self, inline: Inline) -> Result<Vec<u8>, Error> {
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
    }
}
impl Reader {
    pub async fn from_file(path: &str) -> Result<Reader, Error> {
        let file = File::open(path).await.unwrap();
        let mut reader = BufReader::new(file);
        let mut core = Inline::BUFFER;
        reader.read(&mut core).await.unwrap();
        let core = core.into();
        let index = Self::make_index(&mut reader).await;
        Ok(Self {
            reader,
            core,
            index,
        })
    }
    async fn make_index(reader: &mut BufReader<File>) -> HashMap<[u8; SIZE_KIND], KindIndex> {
        let mut index = HashMap::new();
        loop {
            let mut size = [0; SIZE_SIZE];
            let mut kind = [0; SIZE_KIND];
            reader.read(&mut size).await.unwrap();
            reader.read(&mut kind).await.unwrap();
            let size = u64::from_be_bytes(size);
            if kind == Null::KIND {
                return index;
            } else {
                let start = reader.seek(SeekFrom::Current(0)).await.unwrap();
                let elements = Self::make_elements(reader, start + size).await;
                let kind_index = KindIndex {
                    start,
                    size,
                    elements,
                };
                index.insert(kind, kind_index);
                //reader.seek(SeekFrom::Start(start + size)).await.unwrap();
            }
        }
    }
    async fn make_elements(reader: &mut BufReader<File>, end: u64) -> Vec<Element> {
        let mut index = Vec::new();
        loop {
            let mut size = [0; SIZE_SIZE];
            reader.read(&mut size).await.unwrap();
            let size = u64::from_be_bytes(size);
            let start = reader.seek(SeekFrom::Current(0)).await.unwrap();
            let element = Element { start, size };
            index.push(element);
            reader.seek(SeekFrom::Current(size as i64)).await.unwrap();
            if start + size == end {
                return index;
            }
        }
    }
}

#[derive(Debug)]
struct KindIndex {
    start: u64,
    size: u64,
    elements: Vec<Element>,
}
#[derive(Debug)]
struct Element {
    start: u64,
    size: u64,
}
