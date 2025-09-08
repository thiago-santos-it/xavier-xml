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

    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
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
    let test_data = TestNameHandlingWithPrefixSuffix {  some_string: "Some Text".to_string(),   some_int: 10,   some_float: 11.0 };
    let xml = from_obj(&test_data);
    assert!(xml.contains("<xmlJustStringItem>"));

    let obj: TestNameHandlingWithPrefixSuffix = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    
    Ok(())
}

#[test]
fn test_name_handling_ignore_case() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel", prefix="xml_", suffix="_item", no_suffix, no_prefix)]
    struct TestNameHandlingIgnoreCase {
        #[xml(name="just_string", ignore_case)]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let test_data = TestNameHandlingIgnoreCase {  some_string: "Some Text".to_string(),   some_int: 10,   some_float: 11.0 };
    let xml = from_obj(&test_data);

    assert!(xml.contains("<xml_just_string_item>"));

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
    
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
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
        #[xml(tree, name="my_child")]
        pub child: TestNamePrecedenceChildTagFirst
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChildTagFirst {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: TestNamePrecedenceChildTagFirst { id: 2, name: String::from("John Doe Jr") },
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<myChild>"));
    assert!(!xml.contains("<testObjectChild>"));

    let parsed: TestNamePrecedence = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    Ok(())
}

#[test]
fn test_name_precedence_field_second() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object", case="Camel")]
    struct TestNamePrecedenceFieldSecond {
        #[xml(name="custom_id")]
        pub id: u32,
        #[xml(name="name")]
        pub name: String,
        #[xml(flatten)]
        pub child: TestNamePrecedenceChildFieldSecond
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChildFieldSecond {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedenceFieldSecond {
        id: 1,
        name: "John Doe".to_string(),
        child: TestNamePrecedenceChildFieldSecond { id: 2, name: String::from("John Doe Jr") },
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<child>"));
    assert!(!xml.contains("<testObjectChild>"));

    let parsed: TestNamePrecedenceFieldSecond = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

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
        pub child: Vec<TestNamePrecedenceChildInnerVec>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChildInnerVec {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChildInnerVec { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<myChild>"));
    assert!(xml.contains("<goodChild>"));
    assert!(!xml.contains("<testObjectChild>"));

    let parsed: TestNamePrecedence = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

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
        pub child: Vec<TestNamePrecedenceChildStructVec>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChildStructVec {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChildStructVec { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<myChild>"));
    assert!(xml.contains("<testObjectChild>"));

    let parsed: TestNamePrecedence = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

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
        pub child: Vec<TestNamePrecedenceChildFieldVec>
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name="test_object_child", case="Camel")]
    struct TestNamePrecedenceChildFieldVec {
        #[xml(name="child_id")]
        pub id: u32,
        #[xml(name="child_name")]
        pub name: String,
    }

    let test_data = TestNamePrecedence {
        id: 1,
        name: "John Doe".to_string(),
        child: vec![TestNamePrecedenceChildFieldVec { id: 2, name: String::from("John Doe Jr") }],
    };

    let xml = from_obj(&test_data);
    println!("{:?}", xml);
    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<customId>1</customId>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<child>"));
    assert!(xml.contains("<testObjectChild>"));

    let parsed: TestNamePrecedence = from_xml(&xml)?;
    assert_eq!(test_data, parsed);

    Ok(())
}