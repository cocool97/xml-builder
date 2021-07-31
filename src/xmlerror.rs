use std::fmt::Debug;

/// Custom `Result` type thrown by this crate.
pub type Result<T> = std::result::Result<T, XMLError>;

/// Error type thrown by this crate
pub enum XMLError {
    /// Thrown when the given element cannot be inserted into the XML object tree.
    WrongInsert(String),
    /// Thrown when the given `Writer` cannot be written to.
    WriteError(String),
}

impl From<std::io::Error> for XMLError {
    fn from(e: std::io::Error) -> Self {
        XMLError::WriteError(e.to_string())
    }
}

impl Debug for XMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XMLError::WrongInsert(e) => write!(f, "Error encountered during insertion: {}", e),
            XMLError::WriteError(e) => write!(f, "Error encountered during write: {}", e),
        }
    }
}
