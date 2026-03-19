use crate::XMLElement;

/// An enum value representing the types of XML contents
#[derive(Clone)]
pub enum XMLElementContent {
    /// No XML content.
    Empty,

    /// The content is a list of XML elements.
    Mixed(Vec<XMLElementContent>),

    /// The content is an XML element.
    /// Needs to be boxed to avoid infinite size.
    Element(Box<XMLElement>),

    /// The content is a textual string.
    Text(String),
}
