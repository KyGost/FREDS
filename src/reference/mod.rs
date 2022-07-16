use {
    crate::{Data, Error, Inline, Reader as DM, Value},
    std::marker::PhantomData,
};
#[derive(Default, Debug, Clone, Copy)]
pub struct Ref<Kind: Data>(Inline, PhantomData<Kind>);
impl<Kind: Data> Ref<Kind> {
    pub fn new(inline: Inline) -> Self {
        Self(inline, PhantomData)
    }
    pub async fn get(self, dm: &DM) -> Result<Kind, Error> {
        dm.get(self.0).await
    }
    pub async fn set(self, dm: &mut DM) -> Result<(), Error> {
        dm.set(self.0).await
    }
}
impl<Kind: Data, Input: Into<Inline>> From<Input> for Ref<Kind> {
    fn from(from: Input) -> Self {
        let inline = from.into();
        Ref::new(inline)
    }
}
