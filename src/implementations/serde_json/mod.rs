use {
    crate::{Array, Inline, IntoFreds, Map, Null, FREDS},
    serde_json::{Number, Value},
};
impl IntoFreds for Value {
    fn into_freds(self) -> Vec<u8> {
        let mut freds = FREDS::default();
        let core = value_into_inline(&mut freds, self);
        freds.set_core(core);
        freds.into_bytes()
    }
}
pub fn value_into_inline(freds: &mut FREDS, value: Value) -> Inline {
    match value {
        Value::Object(map) => {
            let map = Map {
                data: map
                    .into_iter()
                    .map(|(key, value)| (freds.into_inline(key), value_into_inline(freds, value)))
                    .collect(),
            };
            freds.into_inline(map)
        }
        Value::Array(vec) => {
            let array = Array {
                data: vec
                    .into_iter()
                    .map(|value| value_into_inline(freds, value))
                    .collect(),
            };
            freds.into_inline(array)
        }
        Value::String(string) => freds.into_inline(string),
        Value::Number(number) => number_into_inline(freds, number),
        Value::Bool(bool) => freds.into_inline(bool),
        Value::Null => freds.into_inline(Null),
    }
}

pub fn number_into_inline(freds: &mut FREDS, number: Number) -> Inline {
    if number.is_u64() {
        freds.into_inline(number.as_u64().unwrap())
    } else if number.is_i64() {
        freds.into_inline(number.as_i64().unwrap())
    } else if number.is_f64() {
        freds.into_inline(number.as_f64().unwrap())
    } else {
        panic!()
    }
}

#[test]
fn any() {
    use serde_json::json;
    let json = json!("test");
    json.into_freds();
}
#[test]
fn big() {
    use serde_json::from_str;
    let json: Value = from_str(include_str!("test.json")).unwrap();
    json.into_freds();
}
#[test]
fn to_file() {
    use {serde_json::from_str, std::io::prelude::*};
    let json: Value = from_str(include_str!("test.json")).unwrap();
    let freds = json.into_freds();
    let mut buffer = std::fs::File::create("test.freds").unwrap();
    buffer.write(&freds).unwrap();
}
