#![crate_type = "lib"]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod builder;
mod utils;
mod xml;
mod xmlcontent;
mod xmlelement;
mod xmlerror;
mod xmlversion;

pub use builder::XMLBuilder;
pub use xml::XML;
pub use xmlelement::XMLElement;
pub use xmlerror::{Result, XMLError};
pub use xmlversion::XMLVersion;

use utils::escape_str;
use xmlcontent::XMLElementContent;
