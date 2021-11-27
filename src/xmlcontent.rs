use crate::XMLElement;

/// An enum value representing the types of XML contents
pub(crate) enum XMLElementContent {
    /// No XML content.
    Empty,

    /// The content is a list of XML elements.
    Elements(Vec<XMLElement>),

    /// The content is a textual string.
    Text(String),
}
