use crate::{Result, XMLElement};

/// A trait for converting (and consuming) a value to a `XMLElement`.
///
/// This trait should be implemented by all concrete types needed to be converted to a `XMLElement`.
pub trait ToXMLElement {
    /// Converts the given value to a `XMLElement`.
    fn to_xmlelement(self) -> Result<XMLElement>;
}
