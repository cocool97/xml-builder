#![crate_type = "lib"]
#![forbid(unsafe_code)]

pub use traits::ToXMLElement;
pub use xml::XML;
pub use xmlelement::XMLElement;
pub use xmlerror::{Result, XMLError};
pub use xmlversion::XMLVersion;

use utils::escape_str;
use xmlcontent::XMLElementContent;

mod traits;
mod utils;
mod xml;
mod xmlcontent;
mod xmlelement;
mod xmlerror;
mod xmlversion;
