extern crate core;

mod error;
pub use error::*;
pub type Result<T> = std::result::Result<T, Error>;

pub mod network;
pub mod transaction;
