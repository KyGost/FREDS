use {
    crate::{Array, DataExt, Error, Inline, Map, Null, Write, Writer},
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

#[test]
fn any() {
    use serde_json::json;
    let json = json!("test");
    json.write();
}
#[test]
fn big() {
    use serde_json::from_str;
    let json: Value = from_str(include_str!("test.json")).unwrap();
    json.write();
}
#[test]
fn to_file() {
    use {serde_json::from_str, std::io::prelude::*};
    let json: Value = from_str(include_str!("test.json")).unwrap();
    let writer = json.write();
    let mut buffer = std::fs::File::create("test.writer").unwrap();
    buffer.write(&writer).unwrap();
}
