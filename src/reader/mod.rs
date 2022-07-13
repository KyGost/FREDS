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
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncSeekExt, BufReader, SeekFrom},
    },
};

pub struct Reader<Value: crate::Value> {
    reader: BufReader<File>,
    pub core: Inline,
    data: [Option<Kind<Value>>; 2_usize.pow(SIZE_KIND as u32 * 8)],
}
impl<Value: crate::Value> Reader<Value> {
    pub async fn get(&mut self, inline: Inline) -> Result<Value, Error> {
        match self.get_element(inline).await?.clone() {
            Element::Value(value) => Ok(value),
            Element::Size { start, size } => {
                self.reader.seek(SeekFrom::Start(start)).await.unwrap();
                let mut bytes = vec![0_u8; size as usize];
                self.reader.read(&mut bytes).await.unwrap();
                let value = Value::from_bytes(self, inline.kind, bytes).await?;
                if let Some(data) = &mut self.data[inline.kind[0] as usize] {
                    data.elements[usize::from_be_bytes(inline.data)] = Element::Value(value);
                    if let Some(Element::Value(value)) =
                        data.elements.get(usize::from_be_bytes(inline.data))
                    {
                        Ok(value.clone())
                    } else {
                        unreachable!()
                    }
                } else {
                    unimplemented!()
                }
            }
            Element::Unknown => Value::from_inline(self, inline).await,
        }
    }
    async fn get_element(&self, inline: Inline) -> Result<&Element<Value>, Error> {
        if let Some(Some(kind)) = self.data.get(inline.kind[0] as usize) {
            if let Some(element) = kind.elements.get(usize::from_be_bytes(inline.data)) {
                Ok(element)
            } else {
                unimplemented!()
            }
        } else {
            Ok(&Element::Unknown)
        }
    }
    //async fn get_value(&mut self, inline: Value) -> Result<Value, Error> {}
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
    const NONE: Option<Kind<Value>> = None;
    pub async fn from_file(path: &str) -> Result<Reader<Value>, Error> {
        let file = File::open(path).await.unwrap();
        let mut reader = BufReader::new(file);
        let mut core = Inline::BUFFER;
        reader.read(&mut core).await.unwrap();
        let core = core.into();
        let data = [Self::NONE; 2_usize.pow(SIZE_KIND as u32 * 8)];
        let mut reader = Self { reader, core, data };
        reader.fill_kinds().await;
        Ok(reader)
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
                self.data[kind[0] as usize] = Some(Kind {
                    start,
                    size,
                    elements: Vec::new(),
                });
                self.fill_elements(kind[0] as usize).await;
            }
        }
    }
    async fn fill_elements(&mut self, kind: usize) {
        if let Some(Some(kind)) = self.data.get_mut(kind) {
            let start = kind.start;
            let end = start + kind.size;
            let elements = &mut kind.elements;

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
        } else {
            unimplemented!()
        }
    }
}
