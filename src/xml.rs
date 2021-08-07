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

impl Default for XML {
    fn default() -> Self {
        XML {
            version: XMLVersion::XML1_0,
            encoding: "UTF-8".into(),
            standalone: None,
            sort_attributes: false,
            indent: true,
            root: None,
        }
    }
}

impl XML {
    /// Instantiates a XML object.
    pub fn new() -> Self {
        XML::default()
    }

    /// Sets the XML version attribute field.
    ///
    /// # Arguments
    ///
    /// `version` - An enum value representing the new version to use for the XML.
    pub fn set_xml_version(&mut self, version: XMLVersion) {
        self.version = version;
    }

    /// Sets the XML encoding attribute field.
    ///
    /// # Arguments
    ///
    /// `encoding` - A String representing the encoding to use for the document.
    pub fn set_xml_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }

    /// Sets to `true` the standalone attribute for this XML document.
    pub fn standalone(&mut self) {
        self.standalone = Some(true);
    }

    /// Sets to `false` the standalone attribute for this XML document.
    pub fn not_standalone(&mut self) {
        self.standalone = Some(false);
    }

    /// Enables attributes sorting.
    pub fn enable_attributes_sorting(&mut self) {
        self.sort_attributes = true;
    }

    /// Disables attributes sorting.
    pub fn disable_attributes_sorting(&mut self) {
        self.sort_attributes = false;
    }

    /// Enables XML indentation.
    pub fn enable_indentation(&mut self) {
        self.indent = true;
    }

    /// Disables XML indentation.
    pub fn disable_indentation(&mut self) {
        self.indent = false;
    }

    /// Sets the XML document root element.
    ///
    /// # Arguments
    ///
    /// `element` - An XMLElement qualified as root for the XML document.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Builds an XML document into the specified `Writer`.
    ///
    /// Consumes the XML object.
    pub fn build<W: Write>(self, mut writer: W) -> Result<()> {
        let standalone_attribute = if let Some(standalone) = self.standalone {
            format!(r#" standalone="{}""#, standalone.to_string())
        } else {
            String::default()
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
