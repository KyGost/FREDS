use {
    super::element::Element,
    crate::{Address, Error, Inline},
    tokio::{fs::File, io::BufReader},
};
#[derive(Debug)]
pub struct Kind<Value: crate::Value> {
    pub start: u64,
    pub size: u64,
    pub elements: Vec<Element<Value>>,
}
impl<Value: crate::Value> Kind<Value> {
    pub async fn get(
        &mut self,
        reader: &mut BufReader<File>,
        inline: Inline,
    ) -> Result<&Value, Error> {
        let addr = usize::from_be_bytes(inline.data);
        if self.elements.get(addr).is_none() {
            self.elements[addr] = <Element<Value>>::default();
        }
        if let Some(element) = self.elements.get_mut(addr) {
            let element: &mut Element<Value> = element;
            element.get(reader, inline).await
        } else {
            unreachable!()
        }
    }
}
