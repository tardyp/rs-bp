#![doc = include_str!("../README.md")]



mod parser;
#[macro_use]
mod macros;
mod utils;
mod tests;

pub use parser::BluePrint;
pub use parser::Value;
pub use parser::Module;
pub use parser::Map;
