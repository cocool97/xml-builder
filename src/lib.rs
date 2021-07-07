use std::fmt::Debug;
use std::io::Write;
use std::usize;

type Result<T> = std::result::Result<T, XMLError>;

/// Error type thrown by this crate
pub enum XMLError {
    WrongInsert(String),
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

impl Default for XML {
    fn default() -> Self {
        XML {
            version: XMLVersion::XML1_0,
            encoding: "UTF-8".into(),
            should_render_header: true,
            should_sort_attributes: false,
            custom_header: None,
            root: None,
        }
    }
}

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

    /// The root XML element.
    root: Option<XMLElement>,
}

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

impl XML {
    /// Instantiates a XML object.
    pub fn new() -> Self {
        XML::default()
    }

    /// Does not render XML header for this document.
    ///
    /// Can be used in case of a custom XML implementation such as XMLTV.
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
    pub fn set_custom_header(&mut self, custom_header: String) {
        self.custom_header = Some(custom_header);
    }

    /// Sets the XML document root element.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Builds an XML document into the specified writer implementing Write trait.
    ///
    /// Consumes the XML object.
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
            elem.render(&mut writer, self.should_sort_attributes)?;
        }

        Ok(())
    }
}

/// Structure representing an XML element field.
pub struct XMLElement {
    /// The name of the XML element.
    name: String,

    /// A list of tuple representing (key, value) attributes.
    attributes: Vec<(String, String)>,

    /// A boolean representing whether we want attributes to be sorted.
    should_sort_attributes: Option<bool>,

    /// The content of this XML element.
    content: XMLElementContent,
}

/// An enum value representing the types of XML contents
enum XMLElementContent {
    /// No XML content.
    Empty,

    /// The content is a list of XML elements.
    Elements(Vec<XMLElement>),

    /// The content is a textual string.
    Text(String),
}

impl XMLElement {
    /// Instantiates a XMLElement object.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the XML element.
    pub fn new(name: &str) -> Self {
        XMLElement {
            name: name.into(),
            attributes: Vec::new(),
            should_sort_attributes: None,
            content: XMLElementContent::Empty,
        }
    }

    pub fn set_attribute_sorting(&mut self, should_sort: bool) {
        self.should_sort_attributes = Some(should_sort);
    }

    /// Adds the given name/value attribute to the XMLElement.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the attribute
    /// * `value` - A string slice that holds the value of the attribute
    pub fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.push((name.into(), escape_str(value)));
    }

    /// Adds a new XMLElement child object to the references XMLElement.
    /// Raises a XMLError if trying to add a child to a text XMLElement.
    ///
    /// # Arguments
    ///
    /// * `element` - A XMLElement object to add as child
    pub fn add_child(&mut self, element: XMLElement) -> Result<()> {
        match self.content {
            XMLElementContent::Empty => {
                self.content = XMLElementContent::Elements(vec![element]);
            }
            XMLElementContent::Elements(ref mut e) => {
                e.push(element);
            }
            XMLElementContent::Text(_) => {
                return Err(XMLError::WrongInsert(
                    "Cannot insert child inside an element with text".into(),
                ))
            }
        };

        Ok(())
    }

    /// Adds text content to a XMLElement object.
    /// Raises a XMLError if trying to add text to a non-empty object.
    ///
    /// # Arguments
    ///
    /// * `text` - A string containing the text to add to the object
    pub fn add_text(&mut self, text: String) -> Result<()> {
        match self.content {
            XMLElementContent::Empty => {
                self.content = XMLElementContent::Text(text);
            }
            _ => {
                return Err(XMLError::WrongInsert(
                    "Cannot insert text in a non-empty element".into(),
                ))
            }
        };

        Ok(())
    }

    /// Internal method rendering an attribute list to a String.
    fn attributes_as_string(&self, should_sort: bool) -> String {
        if self.attributes.is_empty() {
            "".to_owned()
        } else {
            let mut attributes = self.attributes.clone();

            // Giving priority to the element boolean, and taking the global xml if not set
            let should_sort_attributes = self.should_sort_attributes.unwrap_or(should_sort);

            if should_sort_attributes {
                attributes.sort();
            }

            let mut result = "".into();

            for (k, v) in &attributes {
                result = format!("{} {}", result, format!(r#"{}="{}""#, k, v));
            }
            result
        }
    }

    /// Renders an XMLElement object into the specified writer implementing Write trait.
    /// Does not take ownership of the object.
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XMLElement to
    pub fn render<W: Write>(&self, writer: &mut W, should_sort: bool) -> Result<()> {
        self.render_level(writer, 0, should_sort)
    }

    /// Internal method rendering and indenting a XMLELement object
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XMLElement to
    /// * `level` - An usize representing the depth of the XML tree. Used to indent the object.
    fn render_level<W: Write>(
        &self,
        writer: &mut W,
        level: usize,
        should_sort: bool,
    ) -> Result<()> {
        let indent = "\t".repeat(level);
        let attributes = self.attributes_as_string(should_sort);

        match &self.content {
            XMLElementContent::Empty => {
                writeln!(writer, "{}<{}{} />", indent, self.name, attributes)?;
            }
            XMLElementContent::Elements(elements) => {
                writeln!(writer, "{}<{}{}>", indent, self.name, attributes)?;
                for elem in elements {
                    elem.render_level(writer, level + 1, should_sort)?;
                }
                writeln!(writer, "{}</{}>", indent, self.name)?;
            }
            XMLElementContent::Text(text) => {
                writeln!(
                    writer,
                    "{}<{}{}>{}</{}>",
                    indent, self.name, attributes, text, self.name
                )?;
            }
        };

        Ok(())
    }
}

fn escape_str(input: &str) -> String {
    input
        .to_owned()
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}