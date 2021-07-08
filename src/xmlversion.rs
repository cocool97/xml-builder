/// Enum representing all currently available XML versions.
pub enum XMLVersion {
    /// XML version 1.0. First definition in 1998.
    XML1_0,

    /// XML version 1.1. First definition in 2004.
    XML1_1,
}

impl ToString for XMLVersion {
    /// Converts the XMLVersion enum value into a string usable in the XML document
    fn to_string(&self) -> String {
        match self {
            XMLVersion::XML1_0 => "1.0".into(),
            XMLVersion::XML1_1 => "1.1".into(),
        }
    }
}
