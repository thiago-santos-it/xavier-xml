use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_name_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
    struct TestNameHandlingBasic {
        #[xml(name="just_string")]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let test_data = TestNameHandlingBasic {
        some_string: "Test Text".to_string(),
        some_int: 42,
        some_float: 3.14,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_object>"));
    assert!(xml.contains("</test_object>"));
    assert!(xml.contains("<xmlJustStringItem>Test Text</xmlJustStringItem>"));
    assert!(xml.contains("<xmlSomeIntItem>42</xmlSomeIntItem>"));
    assert!(xml.contains("<xmlSomeFloatItem>3.14</xmlSomeFloatItem>"));
    
    let parsed: TestNameHandlingBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_name_handling_with_prefix_suffix() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
    struct TestNameHandlingWithPrefixSuffix {
        #[xml(name="just_string")]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let xml = r#"
    <test_object>
        <xmlJustStringItem>Some Text</xmlJustStringItem>
        <xmlSomeIntItem>10</xmlSomeIntItem>
        <xmlSomeFloatItem>11</xmlSomeFloatItem>
    </test_object>"#;
    
    let obj: TestNameHandlingWithPrefixSuffix = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    
    Ok(())
}

#[test]
fn test_name_handling_ignore_case() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
    struct TestNameHandlingIgnoreCase {
        #[xml(name="just_string", ignore_case)]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let xml = r#"
    <test_object>
        <xml_just_stringItem>Some Text</xml_just_stringItem>
        <xmlSomeIntItem>10</xmlSomeIntItem>
        <xmlSomeFloatItem>11</xmlSomeFloatItem>
    </test_object>"#;

    let obj: TestNameHandlingIgnoreCase = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    
    Ok(())
}

#[test]
fn test_name_handling_custom_names() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNameHandlingCustomNames {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="user_name")]
        pub name: String,
        #[xml(name="user_age")]
        pub age: i32,
    }

    let test_data = TestNameHandlingCustomNames {
        id: 1,
        name: "John Doe".to_string(),
        age: 30,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_object>"));
    assert!(xml.contains("</test_object>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<userName>John Doe</userName>"));
    assert!(xml.contains("<userAge>30</userAge>"));
    
    let parsed: TestNameHandlingCustomNames = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}


#[test]
fn test_name_precedence_tag_first() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedence {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        #[xml(name="my_child")]
        pub child: TestNamePrecedenceChild
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChild {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: TestNamePrecedenceChild { id: 2, name: String::from("John Doe Jr") },
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    Ok(())
}

#[test]
fn test_name_precedence_field_second() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedence {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        pub child: TestNamePrecedenceChild
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChild {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: TestNamePrecedenceChild { id: 2, name: String::from("John Doe Jr") },
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    Ok(())
}


#[test]
fn test_name_precedence_inner_vec() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedence {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        #[xml(name="my_child", inner="good_child")]
        pub child: Vec<TestNamePrecedenceChild>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChild {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChild { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    Ok(())
}

#[test]
fn test_name_precedence_struct_vec() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedence {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        #[xml(name="my_child")]
        pub child: Vec<TestNamePrecedenceChild>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChild {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChild { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    Ok(())
}


#[test]
fn test_name_precedence_parent_field_vec() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedence {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        pub child: Vec<TestNamePrecedenceChild>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChild {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChild { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    Ok(())
}