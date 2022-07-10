use {
    crate::{Array, DataExt, Error, Inline, Map, Null, Reader, Write, Writer},
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
        fn value_from_inline(inline: Inline, reader: &mut crate::Reader) -> Result<Value, Error> {
            use crate::{Data, implementations::serde_json::IntoValue};
            Ok(match inline.kind {
                $(<$kind>::KIND => <$kind>::from_inline(inline)?.into_value(reader)?),*,
                _ => Value::Null,
            })
        }
    }
}

trait IntoValue: Sized {
    fn into_value(self, reader: &mut Reader) -> Result<Value, Error>;
}
impl IntoValue for Array {
    fn into_value(self, reader: &mut Reader) -> Result<Value, Error> {
        self.data
            .into_iter()
            .map(|data| value_from_inline(data, reader))
            .collect::<Result<Vec<Value>, Error>>()
            .map(Value::Array)
    }
}
impl IntoValue for Map {
    fn into_value(self, reader: &mut Reader) -> Result<Value, Error> {
        self.data
            .into_iter()
            .map(|(key, value)| Ok((String::from_inline(key)?, value_from_inline(value, reader)?)))
            .collect::<Result<serde_json::Map<String, Value>, Error>>()
            .map(Value::Object)
    }
}
impl IntoValue for String {
    fn into_value(self, _reader: &mut Reader) -> Result<Value, Error> {
        Ok(Value::String(self))
    }
}
impl IntoValue for u64 {
    fn into_value(self, _reader: &mut Reader) -> Result<Value, Error> {
        Ok(Value::Number(self.into()))
    }
}
impl IntoValue for i64 {
    fn into_value(self, _reader: &mut Reader) -> Result<Value, Error> {
        Ok(Value::Number(self.into()))
    }
}
impl IntoValue for f64 {
    fn into_value(self, _reader: &mut Reader) -> Result<Value, Error> {
        Ok(Value::Number(Number::from_f64(self).unwrap()))
    }
}
impl IntoValue for bool {
    fn into_value(self, _reader: &mut Reader) -> Result<Value, Error> {
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
fn big() {
    use serde_json::from_str;
    let json: Value = from_str(include_str!("test.json")).unwrap();
    json.write();
}
#[cfg(feature = "write")]
#[test]
fn to_file() {
    use {serde_json::from_str, std::io::prelude::*};
    let json: Value = from_str(include_str!("test.json")).unwrap();
    let writer = json.write();
    let mut buffer = std::fs::File::create("test.writer").unwrap();
    buffer.write(&writer).unwrap();
}
#[cfg(feature = "read")]
#[test]
fn from_file() {
    unimplemented!()
}
