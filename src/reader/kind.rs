use {
    super::element::Element,
};
#[derive(Debug)]
pub struct Kind<Value: crate::Value> {
    pub start: u64,
    pub size: u64,
    pub elements: Vec<Element<Value>>,
}
