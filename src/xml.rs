use std::io::Write;

use crate::{Result, XMLElement, XMLVersion};

/// Structure representing a XML document.
/// It must be used to create a XML document.
pub struct XML {
    /// The XML version to set for the document.
    ///
    /// Defaults to `XML1.0`.
    version: XMLVersion,

    /// The encoding to set for the document.
    ///
    /// Defaults to `UTF-8`.
    encoding: String,

    /// XML standalone attribute.
    ///
    /// A `None` value indicates no displaying.
    ///
    /// Defaults to `None`
    standalone: Option<bool>,

    /// Whether the XML attributes should be sorted or not.
    ///
    /// Defaults to `false`.
    sort_attributes: bool,

    /// Whether we want to indentate the document.
    ///
    /// Defaults to `true`.
    indent: bool,

    /// The root XML element.
    root: Option<XMLElement>,
}

impl XML {
    pub(crate) fn new(
        version: XMLVersion,
        encoding: String,
        standalone: Option<bool>,
        indent: bool,
        sort_attributes: bool,
    ) -> Self {
        Self {
            version,
            encoding,
            standalone,
            indent,
            sort_attributes,
            root: None,
        }
    }

    /// Sets the XML document root element.
    ///
    /// # Arguments
    ///
    /// `element` - An XMLElement qualified as root for the XML document.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Generates an XML document into the specified `Writer`.
    ///
    /// Consumes the XML object.
    pub fn generate<W: Write>(self, mut writer: W) -> Result<()> {
        let standalone_attribute = match self.standalone {
            Some(_) => r#" standalone="yes""#.to_string(),
            None => String::default(),
        };

        writeln!(
            writer,
            r#"<?xml version="{}" encoding="{}"{}?>"#,
            self.version.to_string(),
            self.encoding,
            standalone_attribute
        )?;

        // And then XML elements if present...
        if let Some(elem) = &self.root {
            elem.render(&mut writer, self.sort_attributes, self.indent)?;
        }

        Ok(())
    }
}
