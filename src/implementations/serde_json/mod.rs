use {
    crate::{Array, Inline, Map, Null, Write, Writer},
    serde_json::{Number, Value},
};
impl Write for Value {
    fn write(self) -> Vec<u8> {
        let mut writer = Writer::default();
        let core = value_into_inline(&mut writer, self);
        writer.set_core(core);
        writer.into_bytes()
    }
}
pub fn value_into_inline(writer: &mut Writer, value: Value) -> Inline {
    match value {
        Value::Object(map) => {
            let map = Map {
                data: map
                    .into_iter()
                    .map(|(key, value)| (writer.into_inline(key), value_into_inline(writer, value)))
                    .collect(),
            };
            writer.into_inline(map)
        }
        Value::Array(vec) => {
            let array = Array {
                data: vec
                    .into_iter()
                    .map(|value| value_into_inline(writer, value))
                    .collect(),
            };
            writer.into_inline(array)
        }
        Value::String(string) => writer.into_inline(string),
        Value::Number(number) => number_into_inline(writer, number),
        Value::Bool(bool) => writer.into_inline(bool),
        Value::Null => writer.into_inline(Null),
    }
}

pub fn number_into_inline(writer: &mut Writer, number: Number) -> Inline {
    if number.is_u64() {
        writer.into_inline(number.as_u64().unwrap())
    } else if number.is_i64() {
        writer.into_inline(number.as_i64().unwrap())
    } else if number.is_f64() {
        writer.into_inline(number.as_f64().unwrap())
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
