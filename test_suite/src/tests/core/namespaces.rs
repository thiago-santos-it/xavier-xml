use xavier::{from_xml, from_obj, PError, XmlDeserializable, XmlSerializable};

#[test]
fn test_namespace_handling_basic_roundtrip() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="xml", name="test_namespace", case="Camel")]
    struct TestNamespaceBasic {
        #[xml(name="just_string")]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let test_data = TestNamespaceBasic {
        some_string: "Test String".to_string(),
        some_int: 42,
        some_float: 3.14,
    };

    let xml = from_obj(&test_data);
    
    assert!(xml.contains("<test_namespace>"));
    assert!(xml.contains("</test_namespace>"));
    assert!(xml.contains("<justString>Test String</justString>"));
    assert!(xml.contains("<someInt>42</someInt>"));
    assert!(xml.contains("<someFloat>3.14</someFloat>"));
    
    let parsed: TestNamespaceBasic = from_xml(&xml)?;
    assert_eq!(test_data.some_string, parsed.some_string);
    assert_eq!(test_data.some_int, parsed.some_int);
    assert_eq!(test_data.some_float, parsed.some_float);
    
    Ok(())
}

#[test]
fn test_namespace_handling_with_namespaces() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="xml", name="test_namespace_complex", case="Camel")]
    struct TestNamespaceComplex {
        #[xml(name="just_string")]
        pub some_string: String,
        pub some_int: i32,
        pub some_float: f32,
    }

    let xml = r#"
    <xml:test_namespace_complex
            xmlns:xml="http://www.w3.org/1999/xml"
            xmlns:xhtml="http://www.w3.org/1999/xhtml">
        <xml:justString>Some Text</xml:justString>
        <xml:someInt>10</xml:someInt>
        <xml:someFloat>11.0</xml:someFloat>
    </xml:test_namespace_complex>"#;

    let obj: TestNamespaceComplex = from_xml(&xml)?;
    assert_eq!(obj.some_string, "Some Text");
    assert_eq!(obj.some_int, 10);
    assert_eq!(obj.some_float, 11.0);
    
    Ok(())
}

#[test]
fn test_namespace_handling_multiple_namespaces() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="app", name="test_namespace_multiple", case="Camel")]
    struct TestNamespaceMultiple {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <app:test_namespace_multiple
            xmlns:app="http://example.com/app"
            xmlns:data="http://example.com/data"
            xmlns:meta="http://example.com/meta">
        <app:id>1</app:id>
        <app:name>Test Name</app:name>
        <app:description>Test Description</app:description>
    </app:test_namespace_multiple>"#;

    let obj: TestNamespaceMultiple = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.description, "Test Description");
    
    Ok(())
}

#[test]
fn test_namespace_handling_nested_structures() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="app", name="test_namespace_child", case="Camel")]
    struct TestNamespaceChild {
        pub id: u32,
        pub name: String,
    }

    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="app", name="test_namespace_parent", case="Camel")]
    struct TestNamespaceParent {
        pub id: u32,
        pub name: String,
        pub children: Vec<TestNamespaceChild>,
    }

    let xml = r#"
    <app:test_namespace_parent
            xmlns:app="http://example.com/app">
        <app:id>1</app:id>
        <app:name>Parent</app:name>
        <app:children>
            <app:test_namespace_child>
                <app:id>1</app:id>
                <app:name>Child 1</app:name>
            </app:test_namespace_child>
            <app:test_namespace_child>
                <app:id>2</app:id>
                <app:name>Child 2</app:name>
            </app:test_namespace_child>
        </app:children>
    </app:test_namespace_parent>"#;

    let obj: TestNamespaceParent = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Parent");
    assert_eq!(obj.children.len(), 2);
    assert_eq!(obj.children[0].id, 1);
    assert_eq!(obj.children[0].name, "Child 1");
    assert_eq!(obj.children[1].id, 2);
    assert_eq!(obj.children[1].name, "Child 2");
    
    Ok(())
}

#[test]
fn test_namespace_handling_attributes() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable)]
    #[xml(ns="app", name="test_namespace_attributes", case="Camel")]
    struct TestNamespaceAttributes {
        #[xml(attribute, name="id")]
        pub id: u32,
        pub name: String,
        pub description: String,
    }

    let xml = r#"
    <app:test_namespace_attributes
            xmlns:app="http://example.com/app"
            app:id="1">
        <app:name>Test Name</app:name>
        <app:description>Test Description</app:description>
    </app:test_namespace_attributes>"#;

    let obj: TestNamespaceAttributes = from_xml(&xml)?;
    assert_eq!(obj.id, 1);
    assert_eq!(obj.name, "Test Name");
    assert_eq!(obj.description, "Test Description");
    
    Ok(())
}