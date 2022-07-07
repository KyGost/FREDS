use crate::{impl_referential, Data, ReferentialData};

impl Data for String {
    const TYPE: [u8; 1] = [b's'];
}
impl ReferentialData for String {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
    fn from_bytes(bytes: Vec<u8>) -> Self {
        Self::from_utf8(bytes).unwrap()
    }
}
impl_referential!(String);
