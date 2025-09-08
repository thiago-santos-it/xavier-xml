use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[test]
fn test_structs_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsBasic {
        pub id: u32,
        pub name: String,
        pub description: String,
        pub active: bool,
    }

    let test_data = TestStructsBasic {
        id: 1,
        name: "Test Struct".to_string(),
        description: "Test Description".to_string(),
        active: true,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestStructsBasic>"));
    assert!(xml.contains("</TestStructsBasic>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Struct</name>"));
    assert!(xml.contains("<description>Test Description</description>"));
    assert!(xml.contains("<active>true</active>"));

    let parsed: TestStructsBasic = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_structs_with_attributes() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    #[xml(name="child")]
    struct TestStructsChild {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        #[xml(value)]
        pub value: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsWithAttributes {
        #[xml(attribute, name="attr")]
        pub attribute: String,
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestStructsChild,
    }

    let test_data = TestStructsWithAttributes {
        attribute: "Attribute Value".to_string(),
        id: 1,
        name: "Test With Attributes".to_string(),
        child: TestStructsChild {
            attribute: "Child Attr".to_string(),
            value: "Child Value".to_string(),
        },
    };

    
    let xml = from_obj(&test_data);
    println!("{:#?}", xml);
    assert!(xml.contains("<TestStructsWithAttributes"));
    assert!(xml.contains("</TestStructsWithAttributes>"));
    assert!(xml.contains("attr=\"Attribute Value\""));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test With Attributes</name>"));
    assert!(xml.contains("attr=\"Child Attr\""));
    assert!(xml.contains(">Child Value<"));

    let parsed: TestStructsWithAttributes = from_xml(&xml)?;
    assert_eq!(test_data.attribute, parsed.attribute);
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.child.attribute, parsed.child.attribute);
    assert_eq!(test_data.child.value, parsed.child.value);
    
    Ok(())
}

#[test]
fn test_structs_unit_struct() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsUnit;

    let test_data = TestStructsUnit;

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestStructsUnit/>"));

    let parsed: TestStructsUnit = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_structs_with_empty_tag() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsUnit;

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsWithEmptyTag {
        pub id: u32,
        pub name: String,
        #[xml(flatten)]
        pub empty: Option<TestStructsUnit>,
    }

    let test_data = TestStructsWithEmptyTag {
        id: 1,
        name: "Test With Empty Tag".to_string(),
        empty: Some(TestStructsUnit),
    };

    
    let xml = from_obj(&test_data);
    
    
    assert!(xml.contains("<TestStructsWithEmptyTag>"));
    assert!(xml.contains("</TestStructsWithEmptyTag>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test With Empty Tag</name>"));
    
    
    let parsed: TestStructsWithEmptyTag = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    
    Ok(())
}

#[test]
fn test_structs_simple_types() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsSimpleTypes {
        pub id: u32,
        pub name: String,
        pub count: i32,
        pub price: f64,
        pub active: bool,
    }

    let test_data = TestStructsSimpleTypes {
        id: 1,
        name: "Simple Types".to_string(),
        count: 42,
        price: 3.14,
        active: true,
    };

    
    let xml = from_obj(&test_data);
    
    
    assert!(xml.contains("<TestStructsSimpleTypes>"));
    assert!(xml.contains("</TestStructsSimpleTypes>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Simple Types</name>"));
    assert!(xml.contains("<count>42</count>"));
    assert!(xml.contains("<price>3.14</price>"));
    assert!(xml.contains("<active>true</active>"));
    
    
    let parsed: TestStructsSimpleTypes = from_xml(&xml)?;
    assert_eq!(test_data, parsed);
    
    Ok(())
}

#[test]
fn test_structs_nested_structures() -> Result<(), PError> {
    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsNestedChild {
        pub id: u32,
        pub name: String,
        pub value: String,
    }

    #[derive(XmlSerializable, XmlDeserializable, Debug, PartialEq)]
    struct TestStructsNested {
        pub id: u32,
        pub name: String,
        pub child: TestStructsNestedChild,
    }

    let test_data = TestStructsNested {
        id: 1,
        name: "Nested Structure".to_string(),
        child: TestStructsNestedChild {
            id: 2,
            name: "Child".to_string(),
            value: "Child Value".to_string(),
        },
    };

    
    let xml = from_obj(&test_data);
    
    
    assert!(xml.contains("<TestStructsNested>"));
    assert!(xml.contains("</TestStructsNested>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Nested Structure</name>"));
    assert!(xml.contains("<child>"));
    assert!(xml.contains("</child>"));
    assert!(xml.contains("<TestStructsNestedChild>"));
    assert!(xml.contains("</TestStructsNestedChild>"));
    
    
    let parsed: TestStructsNested = from_xml(&xml)?;
    assert_eq!(test_data.id, parsed.id);
    assert_eq!(test_data.name, parsed.name);
    assert_eq!(test_data.child.id, parsed.child.id);
    assert_eq!(test_data.child.name, parsed.child.name);
    assert_eq!(test_data.child.value, parsed.child.value);
    
    Ok(())
} 