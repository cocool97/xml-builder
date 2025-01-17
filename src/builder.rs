use crate::{XMLVersion, XML};

/// Builder structure used to generate a custom XML structure.
pub struct XMLBuilder {
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

    /// Whether we want to indentate the document.
    ///
    /// Defaults to `true`.
    indent: bool,

    /// Whether the XML attributes should be sorted or not.
    ///
    /// Defaults to `false`.
    sort_attributes: bool,

    /// Whether we want to break lines or not.
    ///
    /// Defaults to `true`.
    break_lines: bool,

    /// Whether we want to expand empty tags or not.
    ///
    /// Defaults to `false`.
    expand_empty_tags: bool,
}

impl Default for XMLBuilder {
    fn default() -> Self {
        Self {
            version: XMLVersion::XML1_0,
            encoding: "UTF-8".into(),
            standalone: None,
            indent: true,
            sort_attributes: false,
            break_lines: true,
            expand_empty_tags: false,
        }
    }
}

impl XMLBuilder {
    /// Builds a new XMLBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the XML version attribute field.
    ///
    /// # Arguments
    ///
    /// `version` - An enum value representing the new version to use for the XML.
    pub fn version(mut self, version: XMLVersion) -> Self {
        self.version = version;

        self
    }

    /// Sets the XML encoding attribute field.
    ///
    /// # Arguments
    ///
    /// `encoding` - A String representing the encoding to use for the document.
    pub fn encoding(mut self, encoding: String) -> Self {
        self.encoding = encoding;

        self
    }

    /// Sets the standalone attribute for this XML document.
    pub fn standalone(mut self, standalone: Option<bool>) -> Self {
        self.standalone = standalone;

        self
    }

    /// Sets the XML indentation.
    pub fn indent(mut self, indent: bool) -> Self {
        self.indent = indent;

        self
    }

    /// Enables attributes sorting.
    pub fn sort_attributes(mut self, sort: bool) -> Self {
        self.sort_attributes = sort;

        self
    }

    /// Sets whether to break lines.
    pub fn break_lines(mut self, break_lines: bool) -> Self {
        self.break_lines = break_lines;

        self
    }

    /// Sets whether to expand empty tags.
    pub fn expand_empty_tags(mut self, expand_empty_tags: bool) -> Self {
        self.expand_empty_tags = expand_empty_tags;

        self
    }

    /// Builds a new XML structure by consuming self.
    pub fn build(self) -> XML {
        XML::new(
            self.version,
            self.encoding,
            self.standalone,
            self.indent,
            self.sort_attributes,
            self.break_lines,
            self.expand_empty_tags,
        )
    }
}
