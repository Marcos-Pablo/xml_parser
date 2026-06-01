mod combinators;
mod core;
mod primitives;

pub mod xml;

pub use combinators::ParserExt;
pub use core::{ParseResult, Parser};
pub use xml::{Element, element};
