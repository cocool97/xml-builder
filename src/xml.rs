use std::io::Write;

use crate::{Result, XMLElement, XMLVersion};

/// Structure representing a XML document.
/// It must be used to create a XML document.
pub struct XML {
    /// The XML version to set for the document.
    ///
    /// Defaults to `XML1.0`.
    version: XMLVersion,

    /// XML encoding attribute.
    ///
    /// The optional encoding to set for the document.
    ///
    /// Defaults to `None`.
    encoding: Option<String>,

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

    /// Whether we want to break lines or not.
    ///
    /// Defaults to `true`.
    break_lines: bool,

    /// Whether we want to expand empty tags or not.
    ///
    /// Defaults to `false`.
    expand_empty_tags: bool,

    /// The root XML element.
    root: Option<XMLElement>,
}

impl XML {
    pub(crate) const fn new(
        version: XMLVersion,
        encoding: Option<String>,
        standalone: Option<bool>,
        indent: bool,
        sort_attributes: bool,
        break_lines: bool,
        expand_empty_tags: bool,
    ) -> Self {
        Self {
            version,
            encoding,
            standalone,
            indent,
            sort_attributes,
            break_lines,
            expand_empty_tags,
            root: None,
        }
    }

    /// Sets the XML document root element.
    ///
    /// # Arguments
    ///
    /// `element` - An `XMLElement` qualified as root for the XML document.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Generates an XML document into the specified `Writer`.
    ///
    /// Consumes the XML object.
    pub fn generate<W: Write>(self, mut writer: W) -> Result<()> {
        write!(
            writer,
            r#"<?xml version="{}"{encoding}{standalone}?>"#,
            self.version,
            encoding = self
                .encoding
                .map_or_else(String::default, |encoding| format!(
                    " encoding=\"{encoding}\""
                )),
            standalone = if let Some(standalone) = self.standalone
                && standalone
            {
                " standalone=\"yes\"".to_string()
            } else {
                String::default()
            }
        )?;

        if self.break_lines {
            writeln!(writer)?;
        }

        // And then XML elements if present...
        if let Some(elem) = &self.root {
            elem.render(
                &mut writer,
                self.sort_attributes,
                self.indent,
                self.break_lines,
                self.expand_empty_tags,
            )?;
        }

        Ok(())
    }
}
