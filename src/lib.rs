#![doc = include_str!("../Readme.md")]



mod parser;
#[macro_use]
mod macros;
mod utils;
mod tests;
mod string;

pub use parser::BluePrint;
pub use parser::Value;
pub use parser::Module;
pub use parser::Map;
