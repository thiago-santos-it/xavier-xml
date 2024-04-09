use xavier::XmlDeserializable;

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn test() {
    let xml = r#"<?xml version="1.0" encoding = "UTF-8" ?>
                <!DOCTYPE xml SYSTEM "Note.dtd">
               <XMLObject><some_string>10</some_string><some_int>10</some_int><some_float>10</some_float></XMLObject>"#;
    let mut reader = quick_xml::Reader::from_str(&xml);
    println!("Root {:?}", XMLObject::from_xml(&mut reader, None));
}
