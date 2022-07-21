#[derive(Debug)]
pub enum Error {
    Unimplemented,
    ExpectedReferentialType,
    ExpectedInlineType,
    ParseError,
    InvalidKind,
    BadReference,
}
