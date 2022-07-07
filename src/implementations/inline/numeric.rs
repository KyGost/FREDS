use crate::{impl_inline, Data, InlineData};

impl Data for u64 {
    const TYPE: [u8; 1] = [1];
}
impl InlineData for u64 {
    fn into_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
impl_inline!(u64);

impl Data for i64 {
    const TYPE: [u8; 1] = [1];
}
impl InlineData for i64 {
    fn into_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
impl_inline!(i64);

impl Data for f64 {
    const TYPE: [u8; 1] = [1];
}
impl InlineData for f64 {
    fn into_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
impl_inline!(f64);
