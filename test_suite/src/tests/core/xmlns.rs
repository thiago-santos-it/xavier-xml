use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable, namespaces};
use xavier::serialize::namespaces::Namespaces;

#[test]
fn test_xmlns_attributes_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsBasic {
        #[xml(xmlns)]
        pub namespaces: Namespaces,
        #[xml(attribute)]
        pub some_string: String,
        #[xml(attribute)]
        pub some_int: i32,
        pub some_float: f32,
    }

    let ns = namespaces!(xml = "http://www.w3.org/XML/1998/namespace", xhtml = "http://www.w3.org/1999/xhtml");

    let test_data = TestXmlnsBasic {
        namespaces: ns,
        some_string: "Test String".to_string(),
        some_int: 42,
        some_float: 3.14,
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<TestXmlnsBasic"));
    assert!(xml.contains("</TestXmlnsBasic>"));
    assert!(xml.contains("xmlns:xml=\"http://www.w3.org/XML/1998/namespace\""));
    assert!(xml.contains("xmlns:xhtml=\"http://www.w3.org/1999/xhtml\""));
    assert!(xml.contains("some_string=\"Test String\""));
    assert!(xml.contains("some_int=\"42\""));
    assert!(xml.contains("<some_float>3.14</some_float>"));
    
    let parsed: TestXmlnsBasic = from_xml(&xml)?;
    assert_eq!(test_data.some_string, parsed.some_string);
    assert_eq!(test_data.some_int, parsed.some_int);
    assert_eq!(test_data.some_float, parsed.some_float);
    assert!(parsed.namespaces.contains("xmlns:xml=\"http://www.w3.org/XML/1998/namespace\""));
    assert!(parsed.namespaces.contains("xmlns:xhtml=\"http://www.w3.org/1999/xhtml\""));
    Ok(())
}

#[test]
fn test_xmlns_attributes_with_namespaces() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsWithNamespaces {
        #[xml(xmlns)]
        pub namespaces: Namespaces,
        #[xml(attribute)]
        pub some_string: String,
        #[xml(attribute)]
        pub some_int: i32,
        pub some_float: f32,
    }

    let xml = r#"
    <TestXmlnsWithNamespaces
            xmlns:xhtml="http://www.w3.org/1999/xhtml"
            xmlns:xhtml2="http://www.w3.org/1999/xhtml"
            some_string="Some text"
            some_int="11">
        <some_float>10</some_float>
    </TestXmlnsWithNamespaces>"#;
    
    let obj: TestXmlnsWithNamespaces = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some text");
    assert_eq!(obj.some_int, 11);
    assert_eq!(obj.some_float, 10.0);
    assert!(obj.namespaces.contains("xmlns:xhtml=\"http://www.w3.org/1999/xhtml\""));
    
    Ok(())
}

#[test]
fn test_xmlns_attributes_multiple_namespaces() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsMultipleNamespaces {
        #[xml(xmlns)]
        pub namespaces: Namespaces,
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <TestXmlnsMultipleNamespaces
            xmlns:app="http://example.com/app"
            xmlns:data="http://example.com/data"
            xmlns:meta="http://example.com/meta">
        <id>1</id>
        <name>Test Name</name>
        <description>Test Description</description>
    </TestXmlnsMultipleNamespaces>"#;
    
    let obj: TestXmlnsMultipleNamespaces = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.description, "Test Description");
    assert!(obj.namespaces.to_string().contains("xmlns:app="));
    assert!(obj.namespaces.to_string().contains("xmlns:data="));
    assert!(obj.namespaces.to_string().contains("xmlns:meta="));
    
    Ok(())
}

#[test]
fn test_xmlns_attributes_nested_structures() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsNested {
        #[xml(xmlns)]
        pub namespaces: Namespaces,
        pub id: u32,
        pub name: String,
        pub children: Vec<TestXmlnsChild>,
    }

    let xml = r#"
    <TestXmlnsNested
            xmlns:app="http://example.com/app"
            xmlns:child="http://example.com/child">
        <id>1</id>
        <name>Parent</name>
        <children>
            <TestXmlnsChild>
                <id>1</id>
                <name>Child 1</name>
            </TestXmlnsChild>
            <TestXmlnsChild>
                <id>2</id>
                <name>Child 2</name>
            </TestXmlnsChild>
        </children>
    </TestXmlnsNested>"#;
    
    let obj: TestXmlnsNested = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Parent");
    assert_eq!(obj.children.len(), 2);
    assert_eq!(obj.children[0].id, 1);
    assert_eq!(obj.children[0].name, "Child 1");
    assert_eq!(obj.children[1].id, 2);
    assert_eq!(obj.children[1].name, "Child 2");
    assert!(obj.namespaces.to_string().contains("xmlns:app="));
    assert!(obj.namespaces.to_string().contains("xmlns:child="));
    
    Ok(())
}

#[test]
fn test_xmlns_attributes_complex_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug)]
    struct TestXmlnsComplexAttributes {
        #[xml(xmlns)]
        pub namespaces: Namespaces,
        #[xml(attribute, name="id")]
        pub id: u32,
        #[xml(attribute, name="name")]
        pub name: String,
        #[xml(attribute, name="version")]
        pub version: String,
        pub description: String,
    }

    let xml = r#"
    <TestXmlnsComplexAttributes
            xmlns:app="http://example.com/app"
            xmlns:data="http://example.com/data"
            id="1"
            name="Test Name"
            version="1.0">
        <description>Test Description</description>
    </TestXmlnsComplexAttributes>"#;
    
    let obj: TestXmlnsComplexAttributes = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.version, "1.0");
    assert_eq!(obj.description, "Test Description");
    assert!(obj.namespaces.to_string().contains("xmlns:app="));
    assert!(obj.namespaces.to_string().contains("xmlns:data="));
    
    Ok(())
}
