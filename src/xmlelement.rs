use std::io::Write;

use crate::{escape_str, Result, XMLElementContent, XMLError};

/// Structure representing an XML element field.
pub struct XMLElement {
    /// The name of the XML element.
    name: String,

    /// A list of tuple representing (key, value) attributes.
    attributes: Vec<(String, String)>,

    /// A boolean representing whether we want attributes to be sorted.
    ///
    /// If not set, defaults to the root's `XMLELement`.
    sort_attributes: Option<bool>,

    /// The content of this XML element.
    content: XMLElementContent,
}

impl XMLElement {
    /// Instantiates a new XMLElement object.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the XML element.
    pub fn new(name: &str) -> Self {
        XMLElement {
            name: name.into(),
            attributes: Vec::new(),
            sort_attributes: None,
            content: XMLElementContent::Empty,
        }
    }

    /// Enables attributes sorting.
    pub fn enable_attributes_sorting(&mut self) {
        self.sort_attributes = Some(true);
    }

    /// Disables attributes sorting.
    pub fn disable_attributes_sorting(&mut self) {
        self.sort_attributes = Some(false);
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
    ///
    /// Raises `XMLError` if trying to add a child to a text XMLElement.
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
                return Err(XMLError::InsertError(
                    "Cannot insert child inside an element with text".into(),
                ))
            }
        };

        Ok(())
    }

    /// Adds text content to a XMLElement object.
    ///
    /// Raises `XMLError` if trying to add text to a non-empty object.
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
                return Err(XMLError::InsertError(
                    "Cannot insert text in a non-empty element".into(),
                ))
            }
        };

        Ok(())
    }

    /// Internal method rendering attribute list to a String.
    ///
    /// # Arguments
    ///
    /// * `should_sort` - A boolean indicating whether we should sort these atttibutes.
    fn attributes_as_string(&self, should_sort: bool) -> String {
        if self.attributes.is_empty() {
            String::default()
        } else {
            let mut attributes = self.attributes.clone();

            // Giving priority to the element boolean, and taking the global xml if not set
            let should_sort_attributes = self.sort_attributes.unwrap_or(should_sort);

            if should_sort_attributes {
                attributes.sort();
            }

            let mut result = String::new();

            for (k, v) in &attributes {
                result = format!(r#"{} {}="{}""#, result, k, v);
            }
            result
        }
    }

    /// Renders an XMLElement object into the specified writer implementing Write trait.
    ///
    /// Does not take ownership of the object.
    ///
    /// # Arguments
    ///
    /// * `writer` - An object to render the referenced XMLElement to
    pub fn render<W: Write>(
        &self,
        writer: &mut W,
        should_sort: bool,
        should_indent: bool,
        should_break_lines: bool,
        should_expand_empty_tags: bool,
    ) -> Result<()> {
        self.render_level(
            writer,
            0,
            should_sort,
            should_indent,
            should_break_lines,
            should_expand_empty_tags,
        )
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
        should_indent: bool,
        should_break_lines: bool,
        should_expand_empty_tags: bool,
    ) -> Result<()> {
        let indent = match should_indent {
            true => "\t".repeat(level),
            false => "".into(),
        };
        let suffix = match should_break_lines {
            true => "\n",
            false => "",
        };

        let attributes = self.attributes_as_string(should_sort);

        match &self.content {
            XMLElementContent::Empty => match should_expand_empty_tags {
                true => {
                    write!(
                        writer,
                        "{}<{}{}></{}>{}",
                        indent, self.name, attributes, self.name, suffix
                    )?;
                }
                false => {
                    write!(
                        writer,
                        "{}<{}{} />{}",
                        indent, self.name, attributes, suffix
                    )?;
                }
            },
            XMLElementContent::Elements(elements) => {
                write!(writer, "{}<{}{}>{}", indent, self.name, attributes, suffix)?;
                for elem in elements {
                    elem.render_level(
                        writer,
                        level + 1,
                        should_sort,
                        should_indent,
                        should_break_lines,
                        should_expand_empty_tags,
                    )?;
                }
                write!(writer, "{}</{}>{}", indent, self.name, suffix)?;
            }
            XMLElementContent::Text(text) => {
                write!(
                    writer,
                    "{}<{}{}>{}</{}>{}",
                    indent, self.name, attributes, text, self.name, suffix
                )?;
            }
        };

        Ok(())
    }
}
