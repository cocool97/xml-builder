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
            version: "1.0".into(),
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
    version: String,
    encoding: String,
    should_render_header: bool,
    should_sort_attributes: bool,
    custom_header: Option<String>,
    root: Option<XMLElement>,
}

impl XML {
    /// Instantiates a XML object.
    pub fn new() -> Self {
        XML::default()
    }

    /// Does not render XML header for this document
    ///
    /// Can be used in case of a custom XML implementation such as XMLTV
    pub fn set_header_rendering(&mut self, asked: bool) {
        self.should_render_header = asked;
    }

    /// Sets the XML version attribute field.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Sets the XML encoding attribute field.
    pub fn set_encoding(&mut self, encoding: String) {
        self.encoding = encoding;
    }

    /// Sets the header attribute sort.
    pub fn set_attribute_sorting(&mut self, should_sort: bool) {
        self.should_sort_attributes = should_sort;
    }

    /// Sets a custom XML header.
    ///
    /// Be careful, no syntax and semantic verifications are made on this header.
    pub fn set_custom_header(&mut self, header: String) {
        self.custom_header = Some(header);
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
                    self.encoding, self.version
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
    name: String,
    attributes: Vec<(String, String)>,
    should_sort_attributes: Option<bool>,
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

enum XMLElementContent {
    Empty,
    Elements(Vec<XMLElement>),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_default_creation() {
        let xml = XML::new();
        let writer = std::io::sink();
        xml.build(writer).unwrap();
    }

    #[test]
    fn test_xml_file_write() {
        let xml = XML::new();

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();
    }

    #[test]
    fn test_xml_version() {
        let mut xml = XML::new();
        xml.set_version("1.1".into());

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<?xml encoding=\"UTF-8\" version=\"1.1\"?>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...");
    }

    #[test]
    fn test_xml_encoding() {
        let mut xml = XML::new();
        xml.set_encoding("UTF-16".into());

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<?xml encoding=\"UTF-16\" version=\"1.0\"?>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...");
    }

    #[test]
    fn test_custom_header() {
        let mut xml = XML::new();
        let header = "tv generator-info-name=\"my listings generator\"".into(); // XMLTV header example

        xml.set_custom_header(header);

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<tv generator-info-name=\"my listings generator\">\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...");
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

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<?xml encoding=\"UTF-8\" version=\"1.1\"?>
<house rooms=\"2\">
\t<room number=\"1\">This is room number 1</room>
\t<room number=\"2\">This is room number 2</room>
</house>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    // Here the `sort` attribute is set to the root, so everything should be sorted
    #[test]
    fn test_complex_sorted_root_xml() {
        let mut xml = XML::new();
        xml.set_attribute_sorting(true);
        xml.set_version("1.1".into());
        xml.set_encoding("UTF-8".into());

        let mut house = XMLElement::new("house");
        house.add_attribute("rooms", "2");

        for i in 1..=2 {
            let mut room = XMLElement::new("room");
            room.add_attribute("size", &(i * 27).to_string());
            room.add_attribute("number", &i.to_string());
            room.add_text(format!("This is room number {}", i)).unwrap();

            house.add_child(room).unwrap();
        }

        xml.set_root_element(house);

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<?xml encoding=\"UTF-8\" version=\"1.1\"?>
<house rooms=\"2\">
\t<room number=\"1\" size=\"27\">This is room number 1</room>
\t<room number=\"2\" size=\"54\">This is room number 2</room>
</house>\n";
        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
    }

    // Here the `sort` attribute is set to the an element only, so everything should not be sorted
    #[test]
    fn test_complex_sorted_element_xml() {
        let mut xml = XML::new();
        xml.set_version("1.1".into());
        xml.set_encoding("UTF-8".into());

        let mut house = XMLElement::new("house");
        house.add_attribute("rooms", "2");

        for i in 1..=2 {
            let mut room = XMLElement::new("room");
            room.add_attribute("size", &(i * 27).to_string());
            room.add_attribute("city", ["Paris", "LA"][i - 1]);
            room.add_attribute("number", &i.to_string());
            room.add_text(format!("This is room number {}", i)).unwrap();

            if i % 2 == 0 {
                room.set_attribute_sorting(true);
            }

            house.add_child(room).unwrap();
        }

        xml.set_root_element(house);

        let mut writer: Vec<u8> = Vec::new();
        xml.build(&mut writer).unwrap();

        let expected = "<?xml encoding=\"UTF-8\" version=\"1.1\"?>
<house rooms=\"2\">
\t<room size=\"27\" city=\"Paris\" number=\"1\">This is room number 1</room>
\t<room city=\"LA\" number=\"2\" size=\"54\">This is room number 2</room>
</house>\n";

        let res = std::str::from_utf8(&writer).unwrap();

        assert_eq!(res, expected, "Both values does not match...")
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

        xml.build(std::io::stdout()).unwrap();
    }
}
