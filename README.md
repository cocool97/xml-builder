# xml-builder

[![Documentation](https://docs.rs/xml-builder/badge.svg)](https://docs.rs/xml-builder)
[![Latest version](https://img.shields.io/crates/v/xml-builder.svg)](https://crates.io/crates/xml-builder)

This crate allows you to easily create an XML file in a short time by building an object tree. 

Its main advantages are: 

* Fast XML documents creation
* Low size, good for embedeed systems
* 0 dependencies
* Low compilation time
* High configurability of objects

Feel free to contribute to the project and adding your PR's !

## Usage

To use this crate you just need to add this to your `Cargo.toml` file:

```toml
[dependancies]
xml-builder = "*"
```

## Examples

```rust
use xml_builder::{XML, XMLElement};

fn main() {
    let mut xml = XML::new();
    xml.set_attribute_sorting(true);
    xml.set_version("1.1".into());
    xml.set_encoding("UTF-8".into());

    let mut house = XMLElement::new("house");
    house.add_attribute("rooms", "2");

    for i in 1..=2 {
        let mut room = XMLElement::new("room");
        room.add_attribute("price", &(42*i).to_string());
        room.add_attribute("number", &i.to_string());
        room.add_text(format!("This is room number {}", i)).unwrap();

        house.add_child(room).unwrap();
    }

    xml.set_root_element(house);

    let mut stdio = std::io::stdout();
    xml.build(&mut stdio).unwrap();
}
```

This XML content will be displayed:

```xml
<?xml encoding="UTF-8" version="1.1"?>
<house rooms="2">
        <room number="1" price="42">This is room number 1</room>
        <room number="2" price="84">This is room number 2</room>
</house>
```