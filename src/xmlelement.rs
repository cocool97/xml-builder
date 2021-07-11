use std::io::Write;

use crate::{escape_str, Result, XMLElementContent, XMLError};

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

    /// Sets the header attribute sort.
    ///
    /// # Arguments
    ///
    /// `should_sort` - A boolean value indicating whether we want attributes to be sorted.
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
    /// * `should_sort` - A boolean value indicating whether we should attributes sort or not
    /// * `should_indent` - A boolean value indicating whether we should indent this XML element
    pub fn render<W: Write>(
        &self,
        writer: &mut W,
        should_sort: bool,
        should_indent: bool,
    ) -> Result<()> {
        self.render_level(writer, 0, should_sort, should_indent)
    }

    /// Internal method rendering and indenting a XMLELement object
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XMLElement to
    /// * `level` - An usize representing the depth of the XML tree. Used to indent the object.
    /// * `should_sort` - A boolean value indicating whether we should attributes sort or not
    /// * `should_indent` - A boolean value indicating whether we should indent this XML element
    fn render_level<W: Write>(
        &self,
        writer: &mut W,
        level: usize,
        should_sort: bool,
        should_indent: bool,
    ) -> Result<()> {
        let indent = match should_indent {
            true => "\t".repeat(level),
            false => "".into(),
        };

        let attributes = self.attributes_as_string(should_sort);

        match &self.content {
            XMLElementContent::Empty => {
                writeln!(writer, "{}<{}{} />", indent, self.name, attributes)?;
            }
            XMLElementContent::Elements(elements) => {
                writeln!(writer, "{}<{}{}>", indent, self.name, attributes)?;
                for elem in elements {
                    elem.render_level(writer, level + 1, should_sort, should_indent)?;
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
