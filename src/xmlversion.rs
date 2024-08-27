/// Enum representing all currently available XML versions.
pub enum XMLVersion {
    /// XML version 1.0. First definition in 1998.
    XML1_0,

    /// XML version 1.1. First definition in 2004.
    XML1_1,
}

impl std::fmt::Display for XMLVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XMLVersion::XML1_0 => write!(f, "1.0"),
            XMLVersion::XML1_1 => write!(f, "1.1"),
        }
    }
}
