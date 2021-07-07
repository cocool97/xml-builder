use xml_builder::{XMLElement, XMLVersion, XML};

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
    xml.set_version(XMLVersion::XML1_1);

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

#[test]
fn test_complex_xml() {
    let mut xml = XML::new();
    xml.set_version(XMLVersion::XML1_1);
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
    xml.set_version(XMLVersion::XML1_1);
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
    xml.set_version(XMLVersion::XML1_1);
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
