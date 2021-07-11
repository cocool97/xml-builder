use std::io::Write;

use crate::{Result, XMLElement, XMLVersion};

/// Structure representing a XML document.
/// It must be used to create a XML document.
pub struct XML {
    /// The XML version to set for the document.
    version: XMLVersion,

    /// The encoding to set for the document.
    encoding: String,

    /// Whether the XML header should be rendered or not.
    should_render_header: bool,

    /// Whether the XML attributes should be sorted or not.
    should_sort_attributes: bool,

    /// Specifies a custom header to set for the document.
    custom_header: Option<String>,

    /// Whether we want to indentate the document.
    should_indent: bool,

    /// The root XML element.
    root: Option<XMLElement>,
}

impl Default for XML {
    fn default() -> Self {
        XML {
            version: XMLVersion::XML1_0,
            encoding: "UTF-8".into(),
            should_render_header: true,
            should_sort_attributes: false,
            custom_header: None,
            should_indent: true,
            root: None,
        }
    }
}

impl XML {
    /// Instantiates a XML object.
    pub fn new() -> Self {
        XML::default()
    }

    /// Does not render XML header for this document.
    ///
    /// Can be used in case of custom XML implementations such as XMLTV.
    ///
    /// # Arguments
    ///
    /// * `asked` - A boolean value indicating whether we want header rendering.
    pub fn set_header_rendering(&mut self, asked: bool) {
        self.should_render_header = asked;
    }

    /// Sets the XML version attribute field.
    ///
    /// # Arguments
    ///
    /// `version` - An enum value representing the new version to use for the XML.
    pub fn set_version(&mut self, version: XMLVersion) {
        self.version = version;
    }

    /// Sets the XML encoding attribute field.
    ///
    /// # Arguments
    ///
    /// `encoding` - A String representing the encoding to use for the document.
    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }

    /// Sets the header attribute sort.
    ///
    /// # Arguments
    ///
    /// `should_sort` - A boolean value indicating whether we want attributes to be sorted.
    pub fn set_attribute_sorting(&mut self, should_sort: bool) {
        self.should_sort_attributes = should_sort;
    }

    /// Sets a custom XML header.
    ///
    /// Be careful, no syntax and semantic verifications are made on this header.
    ///
    /// # Arguments
    ///
    /// `custom_header` - A String containing the new header value to set for the XML.
    pub fn set_custom_header(&mut self, custom_header: String) {
        self.custom_header = Some(custom_header);
    }

    /// Sets the XML document root element.
    ///
    /// # Arguments
    ///
    /// `element` - An XMLElement qualified as root for the XML document.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Sets the XML indentation.
    ///
    /// Setting a `false` value will lower final XML document size.
    ///
    /// # Arguments
    ///
    /// `should_indent` - A boolean value indicating whether we want indentation for the document.
    pub fn set_document_indentation(&mut self, should_indent: bool) {
        self.should_indent = should_indent;
    }

    /// Builds an XML document into the specified writer implementing Write trait.
    ///
    /// Consumes the XML object.
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XML to
    pub fn build<W: Write>(self, mut writer: W) -> Result<()> {
        // Rendering first XML header line if asked...
        if self.should_render_header {
            if let Some(header) = &self.custom_header {
                writeln!(writer, r#"<{}>"#, header)?;
            } else {
                writeln!(
                    writer,
                    r#"<?xml encoding="{}" version="{}"?>"#,
                    self.encoding,
                    self.version.to_string()
                )?;
            }
        }

        // And then XML elements if present...
        if let Some(elem) = &self.root {
            elem.render(&mut writer, self.should_sort_attributes, self.should_indent)?;
        }

        Ok(())
    }
}
