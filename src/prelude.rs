//! Crate prelude

pub use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Personal preference.
pub use std::format as f;
