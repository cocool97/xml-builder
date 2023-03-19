use std::error::Error;
use std::fmt::{Debug, Display};

/// Custom `Result` type thrown by this crate.
pub type Result<T> = std::result::Result<T, XMLError>;

/// Error type thrown by this crate
pub enum XMLError {
    /// Thrown when the given element cannot be inserted into the XML object tree.
    InsertError(String),
    /// Thrown when the given `Writer` cannot be written to.
    IOError(String),
}

impl From<std::io::Error> for XMLError {
    fn from(e: std::io::Error) -> Self {
        XMLError::IOError(e.to_string())
    }
}

impl Debug for XMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XMLError::InsertError(e) => write!(f, "Error encountered during insertion: {}", e),
            XMLError::IOError(e) => write!(f, "Error encountered during write: {}", e),
        }
    }
}

impl Display for XMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for XMLError {}
