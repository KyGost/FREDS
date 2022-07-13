use {
    crate::{
        data::constants::SIZE_KIND, Array, DataExt, Error, Inline, Map, Null, Reader, Write, Writer,
    },
    serde_json::{Number, Value},
};
impl Write for Value {
    fn write(self) -> Vec<u8> {
        let mut writer = Writer::default();
        let core = value_into_inline(&mut writer, self).unwrap();
        writer.set_core(core);
        writer.into_bytes()
    }
}
pub fn value_into_inline(writer: &mut Writer, value: Value) -> Result<Inline, Error> {
    match value {
        Value::Object(map) => {
            let map = Map {
                data: map
                    .into_iter()
                    .map(|(key, value)| {
                        Ok((key.into_inline(writer)?, value_into_inline(writer, value)?))
                    })
                    .collect::<Result<_, _>>()?,
            };
            map.into_inline(writer)
        }
        Value::Array(vec) => {
            let array = Array {
                data: vec
                    .into_iter()
                    .map(|value| value_into_inline(writer, value))
                    .collect::<Result<_, _>>()?,
            };
            array.into_inline(writer)
        }
        Value::String(string) => string.into_inline(writer),
        Value::Number(number) => number_into_inline(writer, number),
        Value::Bool(bool) => bool.into_inline(writer),
        Value::Null => Null.into_inline(writer),
    }
}

pub fn number_into_inline(writer: &mut Writer, number: Number) -> Result<Inline, Error> {
    if number.is_u64() {
        number.as_u64().unwrap().into_inline(writer)
    } else if number.is_i64() {
        number.as_i64().unwrap().into_inline(writer)
    } else if number.is_f64() {
        number.as_f64().unwrap().into_inline(writer)
    } else {
        panic!()
    }
}

macro_rules! convert_enum {
    [$($kind: ty),*] => {
        async fn value_from_bytes(reader: &mut Reader<Value>, kind: [u8; crate::data::constants::SIZE_KIND], bytes: Vec<u8>) -> Result<Value, Error> {
            use crate::{Data, ReferentialData, implementations::serde_json::IntoValue};
            Ok(match kind {
                Null::KIND => Value::Null,
                $(<$kind>::KIND => <$kind>::from_bytes(bytes)?.into_value(reader).await?),*,
                _ => Value::Null,
            })
        }
    }
}

#[async_trait]
impl crate::Value for Value {
    async fn from_bytes(
        reader: &mut Reader<Value>,
        kind: [u8; SIZE_KIND],
        bytes: Vec<u8>,
    ) -> Result<Self, Error> {
        value_from_bytes(reader, kind, bytes).await
    }
}

use async_trait::async_trait;
#[async_trait]
trait IntoValue: Sized {
    async fn into_value(self, reader: &mut Reader<Value>) -> Result<Value, Error>;
}
#[async_trait]
impl IntoValue for Array {
    async fn into_value(self, reader: &mut Reader<Value>) -> Result<Value, Error> {
        let mut vec = Vec::new();
        for data in self.data.into_iter() {
            let value = reader.get(data).await?;
            vec.push(value);
        }
        Ok(Value::Array(vec))
    }
}
#[async_trait]
impl IntoValue for Map {
    async fn into_value(self, reader: &mut Reader<Value>) -> Result<Value, Error> {
        let mut map = serde_json::Map::new();
        for (key, value) in self.data.into_iter() {
            let key = reader.get(key).await?.as_str().unwrap().to_string();
            let value = reader.get(value).await?;
            map.insert(key, value);
        }
        Ok(Value::Object(map))
    }
}
#[async_trait]
impl IntoValue for String {
    async fn into_value(self, _reader: &mut Reader<Value>) -> Result<Value, Error> {
        Ok(Value::String(self))
    }
}
#[async_trait]
impl IntoValue for u64 {
    async fn into_value(self, _reader: &mut Reader<Value>) -> Result<Value, Error> {
        Ok(Value::Number(self.into()))
    }
}
#[async_trait]
impl IntoValue for i64 {
    async fn into_value(self, _reader: &mut Reader<Value>) -> Result<Value, Error> {
        Ok(Value::Number(self.into()))
    }
}
#[async_trait]
impl IntoValue for f64 {
    async fn into_value(self, _reader: &mut Reader<Value>) -> Result<Value, Error> {
        Ok(Value::Number(Number::from_f64(self).unwrap()))
    }
}
#[async_trait]
impl IntoValue for bool {
    async fn into_value(self, _reader: &mut Reader<Value>) -> Result<Value, Error> {
        Ok(Value::Bool(self))
    }
}

convert_enum![Array, Map, String, u64, i64, f64, bool];

#[cfg(feature = "write")]
#[test]
fn any() {
    use serde_json::json;
    let json = json!("test");
    json.write();
}
#[cfg(feature = "write")]
#[test]
fn write_book() {
    use serde_json::from_str;
    let json: Value = from_str(include_str!("test_book.json")).unwrap();
    json.write();
}
#[cfg(feature = "write")]
#[test]
fn write_misc() {
    use serde_json::from_str;
    let json: Value = from_str(include_str!("test_misc.json")).unwrap();
    json.write();
}
#[cfg(feature = "write")]
#[test]
fn to_file_book() {
    use {serde_json::from_str, std::io::prelude::*};
    let json: Value = from_str(include_str!("test_book.json")).unwrap();
    let writer = json.write();
    let mut buffer = std::fs::File::create("test_book.freds").unwrap();
    buffer.write(&writer).unwrap();
}
#[cfg(feature = "write")]
#[test]
fn to_file_misc() {
    use {serde_json::from_str, std::io::prelude::*};
    let json: Value = from_str(include_str!("test_misc.json")).unwrap();
    let writer = json.write();
    let mut buffer = std::fs::File::create("test_misc.freds").unwrap();
    buffer.write(&writer).unwrap();
}
#[cfg(feature = "write")]
#[test]
fn to_file_simple() {
    use {serde_json::json, std::io::prelude::*};
    let json: Value = json!({"a": "Test!"});
    let writer = json.write();
    let mut buffer = std::fs::File::create("test_simple.freds").unwrap();
    buffer.write(&writer).unwrap();
}
#[cfg(feature = "read")]
#[test]
fn from_file_simple() {
    use {serde_json::json, tokio::runtime::Runtime};
    let runtime = Runtime::new().unwrap();
    let mut reader = runtime
        .block_on(Reader::from_file("test_simple.freds"))
        .unwrap();
    let json: Value = runtime.block_on(reader.get(reader.core)).unwrap();
    let compare: Value = json!({"a": "Test!"});
    assert_eq!(json, compare);
}
#[cfg(feature = "read")]
#[test]
fn from_file_book() {
    use {serde_json::from_str, tokio::runtime::Runtime};
    let runtime = Runtime::new().unwrap();
    let mut reader = runtime
        .block_on(Reader::from_file("test_book.freds"))
        .unwrap();
    let json: Value = runtime.block_on(reader.get(reader.core)).unwrap();
    let compare: Value = from_str(include_str!("test_book.json")).unwrap();
    assert!(json == compare);
}
#[cfg(feature = "read")]
#[test]
fn from_file_misc() {
    use {serde_json::from_str, tokio::runtime::Runtime};
    let runtime = Runtime::new().unwrap();
    let mut reader = runtime
        .block_on(Reader::from_file("test_misc.freds"))
        .unwrap();
    let json: Value = runtime.block_on(reader.get(reader.core)).unwrap();
    let compare: Value = from_str(include_str!("test_misc.json")).unwrap();
    assert!(json == compare);
}
