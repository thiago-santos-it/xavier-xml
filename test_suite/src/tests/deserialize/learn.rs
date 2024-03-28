use xavier::deserialize::parser::root::XmlRoot;


#[test]
fn test() {
    let xml = r#"<?xml version="1.0" encoding = "UTF-8" ?>
                <!DOCTYPE xml SYSTEM "Note.dtd">
                <tag1 s:att1 = "test">
                    <tag2>
                        <!--Test comment-->Test
                        <tag3><!--Test comment-->Test</tag3>
                    </tag2>
                    <tag2>Test 2</tag2>
             </tag1>"#;
    println!("Root {:?}", XmlRoot::parser(xml));
}
