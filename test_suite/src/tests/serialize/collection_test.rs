use xavier::{from_obj, XmlSerializable};

#[derive(XmlSerializable)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    pub children: Vec<Child>
}

#[test]
fn serialize() {
    let should = r#"<object><fieldA>Some Text</fieldA><children><my_child><child_field_a>Other value A</child_field_a></my_child><my_child><child_field_a>Other value B</child_field_a></my_child></children></object>"#;
    let xml = XMLObject { 
        field_a: "Some Text".to_string(), 
        children: vec![
            Child { child_field_a: "Other value A".to_string() },
            Child { child_field_a: "Other value B".to_string() }
        ]
    };
    assert_eq!(from_obj(&xml), should);
}

#[test]
fn serialize_empty_collection() {
    let xml = XMLObject { 
        field_a: "Some Text".to_string(), 
        children: vec![]
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<fieldA>Some Text</fieldA>"));
    assert!(result.contains("<children></children>"));
}

#[test]
fn serialize_large_collection() {
    let mut children = Vec::new();
    for i in 0..100 {
        children.push(Child { 
            child_field_a: format!("Child {} Value", i) 
        });
    }
    
    let xml = XMLObject { 
        field_a: "Large Collection Test".to_string(), 
        children
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<fieldA>Large Collection Test</fieldA>"));
    assert!(result.contains("<child_field_a>Child 0 Value</child_field_a>"));
    assert!(result.contains("<child_field_a>Child 99 Value</child_field_a>"));
}

#[derive(XmlSerializable)]
#[xml(name="nested_child")]
struct NestedChild {
    pub nested_field: String,
    pub sub_children: Vec<Child>,
}

#[derive(XmlSerializable)]
#[xml(name="nested_object")]
struct NestedObject {
    pub field_a: String,
    pub nested_children: Vec<NestedChild>,
}

#[test]
fn serialize_nested_collections() {
    let nested_children = vec![
        NestedChild {
            nested_field: "Nested 1".to_string(),
            sub_children: vec![
                Child { child_field_a: "Sub Child 1A".to_string() },
                Child { child_field_a: "Sub Child 1B".to_string() },
            ]
        },
        NestedChild {
            nested_field: "Nested 2".to_string(),
            sub_children: vec![
                Child { child_field_a: "Sub Child 2A".to_string() },
            ]
        }
    ];
    
    let xml = NestedObject {
        field_a: "Nested Test".to_string(),
        nested_children
    };
    
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Nested Test</field_a>"));
    assert!(result.contains("<nested_field>Nested 1</nested_field>"));
    assert!(result.contains("<nested_field>Nested 2</nested_field>"));
    assert!(result.contains("<child_field_a>Sub Child 1A</child_field_a>"));
    assert!(result.contains("<child_field_a>Sub Child 1B</child_field_a>"));
    assert!(result.contains("<child_field_a>Sub Child 2A</child_field_a>"));
}

#[derive(XmlSerializable)]
struct XMLObjectWithOption {
    pub field_a: String,
    pub children: Option<Vec<Child>>
}

#[test]
fn serialize_option_some() {
    let xml = XMLObjectWithOption { 
        field_a: "Some Text".to_string(), 
        children: Some(vec![
            Child { child_field_a: "Other value A".to_string() },
            Child { child_field_a: "Other value B".to_string() }
        ])
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Some Text</field_a>"));
    assert!(result.contains("<children>"));
    assert!(result.contains("<child_field_a>Other value A</child_field_a>"));
    assert!(result.contains("<child_field_a>Other value B</child_field_a>"));
}

#[test]
fn serialize_option_none() {
    let xml = XMLObjectWithOption { 
        field_a: "Some Text".to_string(), 
        children: None
    };
    let result = from_obj(&xml);
    assert!(!result.is_empty());
    assert!(result.contains("<field_a>Some Text</field_a>"));
    assert!(!result.contains("<children></children>"));
}

#[derive(XmlSerializable)]
struct CustomInnerTestStruct {
    pub id: u64,
    pub name: String,
    #[xml(inner="item")]
    pub data: Vec<String>,
    #[xml(inner="element")]
    pub items: Vec<String>,
    #[xml(inner="value")]
    pub scores: Vec<u32>,
}

#[test]
fn serialize_custom_inner_tags() {
    let test_data = CustomInnerTestStruct {
        id: 1,
        name: "CustomTest".to_string(),
        data: vec!["data1".to_string(), "data2".to_string()],
        items: vec!["item1".to_string(), "item2".to_string()],
        scores: vec![100, 200, 300],
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that all internal tags are correct
    assert!(xml.contains("<data><item>data1</item><item>data2</item></data>"));
    assert!(xml.contains("<items><element>item1</element><element>item2</element></items>"));
    assert!(xml.contains("<scores><value>100</value><value>200</value><value>300</value></scores>"));
}

#[test]
fn serialize_various_collection_types_with_inner() {
    #[derive(XmlSerializable)]
    struct CollectionTestStruct {
        #[xml(inner="string")]
        pub strings: Vec<String>,
        #[xml(inner="number")]
        pub numbers: Vec<i32>,
        #[xml(inner="float")]
        pub floats: Vec<f64>,
        #[xml(inner="boolean")]
        pub booleans: Vec<bool>,
    }
    
    let test_data = CollectionTestStruct {
        strings: vec!["hello".to_string(), "world".to_string()],
        numbers: vec![1, 2, 3, 4, 5],
        floats: vec![1.1, 2.2, 3.3],
        booleans: vec![true, false, true],
    };
    
    let xml = from_obj(&test_data);
    
    // Verify that all internal tags are correct
    assert!(xml.contains("<strings><string>hello</string><string>world</string></strings>"));
    assert!(xml.contains("<numbers><number>1</number><number>2</number><number>3</number><number>4</number><number>5</number></numbers>"));
    assert!(xml.contains("<floats><float>1.1</float><float>2.2</float><float>3.3</float></floats>"));
    assert!(xml.contains("<booleans><boolean>true</boolean><boolean>false</boolean><boolean>true</boolean></booleans>"));
}
