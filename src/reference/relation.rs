use crate::{Error, Inline, Ref, Value, DM};

#[derive(Debug, Clone)]
pub enum Relation {
    Index(usize),
    Field(String),
    Compare(Inline),
    SameAs(Inline),
}

pub trait RefRelation: Sized {
    fn get_ref<Kind: Value>(
        _dm: &DM,
        _reference: Self,
        _relation: Relation,
    ) -> Result<Option<Ref<Kind>>, Error> {
        Err(Error::Unimplemented)
    }
}
