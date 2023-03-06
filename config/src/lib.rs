mod args;
mod config;
mod error;
mod toml;

pub use self::toml::*;
pub use args::*;
pub use config::*;
pub use error::*;

pub type Result<T> = std::result::Result<T, Error>;
