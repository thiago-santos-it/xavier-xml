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
    assert!(result.contains("<children></children>"));
}
