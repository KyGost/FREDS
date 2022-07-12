use {
    super::element::Element,
    crate::{Address, Error, Inline, Reader},
};
#[derive(Debug)]
pub struct Kind<Value: crate::Value> {
    start: u64,
    size: u64,
    elements: Vec<Element<Value>>,
}
impl<Value: crate::Value> Kind<Value> {
    pub async fn get(
        &mut self,
        reader: &mut Reader<Value>,
        inline: Inline,
    ) -> Result<Value, Error> {
        self.elements
            .get(inline.data.into())
            .unwrap_or_default()
            .get(reader, inline)
    }
}
