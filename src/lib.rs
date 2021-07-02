use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
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

impl Display for XML {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: Vec<u8> = Vec::new();
        self.render(&mut s)
            .expect("Failure writing output to Vec<u8>");

        write!(f, "{}", unsafe { String::from_utf8_unchecked(s) })
    }
}

impl Default for XML {
    fn default() -> Self {
        XML {
            version: "1.0".into(),
            encoding: "UTF-8".into(),
            custom_header: None,
            root: None,
        }
    }
}

/// Structure representing a XML document.
/// It must be used to create a XML document.
pub struct XML {
    version: String,
    encoding: String,
    custom_header: Option<String>,
    root: Option<XMLElement>,
}

impl XML {
    /// Instantiates a XML object.
    pub fn new() -> Self {
        XML::default()
    }

    /// Sets the XML version attribute field.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Sets the XML encoding attribute field.
    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }

    /// Sets a custom XML header.
    /// Be careful, no syntax and semantic verifications are made on this header.
    pub fn set_custom_header(&mut self, header: String) {
        self.custom_header = Some(header);
    }

    /// Sets the XML document root element.
    pub fn set_root_element(&mut self, element: XMLElement) {
        self.root = Some(element);
    }

    /// Renders an XML document into the specified writer implementing Write trait.
    /// Does not take ownership of the object.
    pub fn render<W: Write>(&self, mut writer: W) -> Result<()> {
        // Rendering first XML header line...
        if let Some(header) = &self.custom_header {
            writeln!(writer, r#"<{}>"#, header)?;
        } else {
            writeln!(
                writer,
                r#"<?xml version="{}" encoding="{}"?>"#,
                self.version, self.encoding
            )?;
        }

        // And then XML elements if present...
        if let Some(elem) = &self.root {
            elem.render(&mut writer)?;
        }

        Ok(())
    }

    /// Builds an XML document into the specified writer implementing Write trait.
    /// Consumes the XML object.
    pub fn build<W: Write>(self, writer: W) -> Result<()> {
        self.render(writer)
    }
}

/// Structure representing an XML element field.
pub struct XMLElement {
    name: String,
    attributes: HashMap<String, String>,
    content: XMLElementContent,
}

impl XMLElement {
    /// Instantiates a XMLElement object.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the XML element.
    pub fn new(name: &str) -> Self {
        XMLElement {
            name: name.to_owned(),
            attributes: HashMap::new(),
            content: XMLElementContent::Empty,
        }
    }

    /// Adds the given name/value attribute to the XMLElement.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the attribute
    /// * `value` - A string slice that holds the value of the attribute
    pub fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_owned(), escape_str(value));
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
    fn attributes_as_string(&self) -> String {
        if self.attributes.is_empty() {
            "".to_owned()
        } else {
            let mut result = "".to_owned();
            for (k, v) in &self.attributes {
                result = result + &format!(r#" {}="{}""#, k, v);
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
    pub fn render<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.render_level(writer, 0)
    }

    /// Internal method rendering and indenting a XMLELement object
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XMLElement to
    /// * `level` - An usize representing the depth of the XML tree. Used to indent the object. 
    fn render_level<W: Write>(&self, writer: &mut W, level: usize) -> Result<()> {
        let indent = "\t".repeat(level);

        match &self.content {
            XMLElementContent::Empty => {
                writeln!(
                    writer,
                    "{}<{}{} />",
                    indent,
                    self.name,
                    self.attributes_as_string()
                )?;
            }
            XMLElementContent::Elements(elements) => {
                writeln!(
                    writer,
                    "{}<{}{}>",
                    indent,
                    self.name,
                    self.attributes_as_string()
                )?;
                for elem in elements {
                    elem.render_level(writer, level + 1)?;
                }
                writeln!(writer, "{}</{}>", indent, self.name)?;
            }
            XMLElementContent::Text(text) => {
                writeln!(
                    writer,
                    "{}<{}{}>{}</{}>",
                    indent,
                    self.name,
                    self.attributes_as_string(),
                    text,
                    self.name
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

enum XMLElementContent {
    Empty,
    Elements(Vec<XMLElement>),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_xml_default_creation() {
        let xml = XML::new();
        let writer = std::io::sink();
        xml.build(writer).unwrap();
    }

    #[test]
    fn test_xml_file_write() {
        let xml = XML::new();

        let vec: Vec<u8> = Vec::new();
        let writer = Cursor::new(vec);

        xml.build(writer).unwrap();
    }

    #[test]
    fn test_xml_version() {
        let mut xml = XML::new();
        xml.set_version("1.1".into());

        let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>\n";

        assert_eq!(
            format!("{}", xml),
            expected,
            "Both values does not match..."
        );
    }

    #[test]
    fn test_xml_encoding() {
        let mut xml = XML::new();
        xml.set_encoding("UTF-16".into());

        let expected = "<?xml version=\"1.0\" encoding=\"UTF-16\"?>\n";

        assert_eq!(
            format!("{}", xml),
            expected,
            "Both values does not match..."
        );
    }

    #[test]
    fn test_custom_header() {
        let mut xml = XML::new();
        let header = "tv generator-info-name=\"my listings generator\"".into(); // XMLTV header example

        xml.set_custom_header(header);

        let expected = "<tv generator-info-name=\"my listings generator\">\n";

        assert_eq!(
            format!("{}", xml),
            expected,
            "Both values does not match..."
        );
    }

    #[test]
    fn test_complex_xml() {
        let mut xml = XML::new();
        xml.set_version("1.1".into());
        xml.set_encoding("UTF-8".into());

        let mut house = XMLElement::new("house");
        house.add_attribute("rooms", "2");

        for i in 1..=2 {
            let mut room = XMLElement::new("room");
            room.add_attribute("number", &i.to_string());
            room.add_text(format!("This is room number {}", i)).unwrap();

            house.add_child(room).unwrap();
        }

        xml.set_root_element(house);

        let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>
<house rooms=\"2\">
\t<room number=\"1\">This is room number 1</room>
\t<room number=\"2\">This is room number 2</room>
</house>\n";

        assert_eq!(
            format!("{}", xml),
            expected,
            "Both values does not match..."
        )
    }

    #[test]
    #[should_panic]
    fn test_panic_child_for_text_element() {
        let xml = XML::new();

        let mut xml_child = XMLElement::new("panic");
        xml_child
            .add_text("This should panic right after this...".into())
            .unwrap();

        let xml_child2 = XMLElement::new("sorry");
        xml_child.add_child(xml_child2).unwrap();

        xml.render(std::io::stdout()).unwrap();
    }
}
