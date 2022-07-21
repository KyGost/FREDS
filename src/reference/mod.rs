mod relation;
pub use relation::*;
use {
    crate::{Data, Error, Inline, Value, DM},
    std::marker::PhantomData,
};
#[derive(Default, Debug, Clone, Copy)]
pub struct Ref<Kind: Value> {
    pub inline: Inline,
    p_kind: PhantomData<Kind>,
}
impl<Kind: Value> Ref<Kind> {
    pub fn new(inline: Inline) -> Self {
        Self {
            inline,
            p_kind: PhantomData,
        }
    }
    pub async fn get(self, dm: &DM) -> Result<Kind, Error> {
        dm.get(self.0).await
    }
    pub async fn set(self, dm: &mut DM) -> Result<(), Error> {
        dm.set(self.0).await
    }
}
impl<Kind: Value, Input: Into<Inline>> From<Input> for Ref<Kind> {
    fn from(from: Input) -> Self {
        let inline = from.into();
        Ref::new(inline)
    }
}
