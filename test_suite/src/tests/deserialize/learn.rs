use quick_xml::events::Event;
use quick_xml::reader::Reader;


#[test]
fn test() {
    let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test<tag3><!--Test comment-->Test</tag3></tag2>
                <tag2>Test 2</tag2>
             </tag1>"#;
    let mut reader = Reader::from_str(xml);

    let mut count = 0;
    let mut txt = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"tag1" => println!("attributes values: {:?}",
                                        e.attributes().map(|a| a.unwrap().value)
                                            .collect::<Vec<_>>()),
                    b"tag2" => count += 1,
                    b"tag3" => { println!("tag 3") }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {},
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            Ok(Event::Empty(e)) => {},
            Ok(Event::Comment(e)) => {},
            Ok(Event::CData(e)) => {},
            Ok(Event::Decl(e)) => {},
            Ok(Event::PI(e)) => {},
            Ok(Event::DocType(e)) => {}
        }
    }
}
