
mod consts;
pub use self::consts::*;

pub mod config;
pub mod game;

pub extern crate anyhow;
pub extern crate tracing;

pub extern crate cached;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

