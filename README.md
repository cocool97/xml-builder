# xml-builder

[![Documentation](https://docs.rs/xml-builder/badge.svg)](https://docs.rs/xml-builder)
[![Latest version](https://img.shields.io/crates/v/xml-builder.svg)](https://crates.io/crates/xml-builder)
[![dependency status](https://deps.rs/repo/github/cocool97/xml-builder/status.svg)](https://deps.rs/repo/github/cocool97/xml-builder)
[![codecov](https://codecov.io/gh/cocool97/xml-builder/branch/master/graph/badge.svg?token=2PMZ6D9E5M)](https://codecov.io/gh/cocool97/xml-builder)

This crate allows you to easily create an XML file in a short time by building a highly-configurable object tree. 

## Main advantages

This crate offers many advantages over other XML-building crates :

* Fast and easy XML documents creation
* Low size, suits fine for embedeed systems
* Does not depend on other crates
* Highly configurable
* No unsafe code, it integrates the `#![forbid(unsafe_code)]` lint directive

## Main features

Using this crate can bring you many useful features :

* Element attributes sorting
* XML indentation, or not
* Custom XML versions
* Custom XML encodings

## Usage

To use this crate you just need to add this to your `Cargo.toml` file:

```toml
[dependencies]
xml-builder = "*"
```

## Examples

```rust
use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

fn main() {
    let mut xml = XMLBuilder::new()
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .build();

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
    xml.generate(&mut writer).unwrap();
}
```

This following XML content will then be displayed:

```xml
<?xml encoding="UTF-8" version="1.1"?>
<house rooms="2">
        <room number="1" price="42">This is room number 1</room>
        <room number="2" price="84">This is room number 2</room>
</house>
```