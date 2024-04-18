use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
struct XMLObject {
    pub _some_string: String,
    pub _some_int: i32,
    pub _some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = r#"
               <XMLObject>
                    <_some_int>10</_some_int>
                    <_some_float>11.1</_some_float>
               </XMLObject>"#;
    let result: Result<XMLObject, PError> = from_xml(&xml);
    if let Err(_error) = result {
        //println!("{:?}", _error);
        assert!(true)
    } else {
        assert!(false)
    }
    Ok(())
}
