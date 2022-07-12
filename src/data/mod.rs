pub mod constants;
mod inline;
mod traits;

pub use {inline::Inline, traits::*};

pub type Address = [u8; constants::SIZE_INLINE];
