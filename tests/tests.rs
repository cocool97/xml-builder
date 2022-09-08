use xml_builder::{XMLBuilder, XMLElement, XMLVersion};

#[test]
fn test_xml_default_creation() {
    let xml = XMLBuilder::new().build();
    let writer = std::io::sink();
    xml.generate(writer).unwrap();
}

#[test]
fn test_xml_file_write() {
    let xml = XMLBuilder::new().build();

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();
}

#[test]
fn test_xml_version() {
    let xml = XMLBuilder::new().version(XMLVersion::XML1_1).build();

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>\n";
    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...");
}

#[test]
fn test_xml_encoding() {
    let xml = XMLBuilder::new().encoding("UTF-16".into()).build();

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.0\" encoding=\"UTF-16\"?>\n";
    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...");
}

#[test]
fn test_indent() {
    let mut xml = XMLBuilder::new().indent(false).build();

    let mut root = XMLElement::new("root");
    let first_element_inside = XMLElement::new("indentation");
    let second_element_inside = XMLElement::new("indentation");

    root.add_child(first_element_inside).unwrap();
    root.add_child(second_element_inside).unwrap();

    xml.set_root_element(root);

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<root>
<indentation />
<indentation />
</root>\n";
    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...");
}

#[test]
fn test_xml_version_1_0() {
    let xml = XMLBuilder::new().version(XMLVersion::XML1_0).build();

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...");
}

#[test]
fn test_xml_version_1_1() {
    let xml = XMLBuilder::new().version(XMLVersion::XML1_1).build();

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>\n";
    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...");
}

#[test]
#[should_panic]
fn test_panic_child_for_text_element() {
    let xml = XMLBuilder::new().build();

    let mut xml_child = XMLElement::new("panic");
    xml_child
        .add_text("This should panic right after this...".into())
        .unwrap();

    let xml_child2 = XMLElement::new("sorry");
    xml_child.add_child(xml_child2).unwrap();

    xml.generate(std::io::stdout()).unwrap();
}

#[test]
fn test_complex_xml() {
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

    let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>
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
    let mut xml = XMLBuilder::new()
        .sort_attributes(true)
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .build();

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
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\"?>
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
    let mut xml = XMLBuilder::new()
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .standalone(Some(true))
        .build();

    let mut house = XMLElement::new("house");
    house.add_attribute("rooms", "2");

    for i in 1..=2 {
        let mut room = XMLElement::new("room");
        room.add_attribute("size", &(i * 27).to_string());
        room.add_attribute("city", ["Paris", "LA"][i - 1]);
        room.add_attribute("number", &i.to_string());
        room.add_text(format!("This is room number {}", i)).unwrap();

        if i % 2 == 0 {
            room.enable_attributes_sorting();
        }

        house.add_child(room).unwrap();
    }

    xml.set_root_element(house);

    let mut writer: Vec<u8> = Vec::new();
    xml.generate(&mut writer).unwrap();

    let expected = "<?xml version=\"1.1\" encoding=\"UTF-8\" standalone=\"yes\"?>
<house rooms=\"2\">
\t<room size=\"27\" city=\"Paris\" number=\"1\">This is room number 1</room>
\t<room city=\"LA\" number=\"2\" size=\"54\">This is room number 2</room>
</house>\n";

    let res = std::str::from_utf8(&writer).unwrap();

    assert_eq!(res, expected, "Both values does not match...")
}
