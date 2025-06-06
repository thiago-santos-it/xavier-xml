use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: Option<Box<Child>>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectRecursion {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}

#[test]
fn deserialize_recursion() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <my_child attr="Outer Attr">
            <child_field_a>Outer Value</child_field_a>
            <my_child attr="Inner Attr">
                <child_field_a>Inner Value</child_field_a>
                <my_child attr="Deep Attr">
                    <child_field_a>Deep Value</child_field_a>
                </my_child>
            </my_child>
        </my_child>
    </object>"#;
    let obj: XMLObjectRecursion = from_xml(&xml)?;
    println!("{:?}", obj);
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.child.attribute, "Outer Attr");
    assert_eq!(obj.child.child_field_a, "Outer Value");
    let inner = obj.child.inner.unwrap();
    assert_eq!(inner.attribute, "Inner Attr");
    assert_eq!(inner.child_field_a, "Inner Value");
    let deep = inner.inner.unwrap();
    assert_eq!(deep.attribute, "Deep Attr");
    assert_eq!(deep.child_field_a, "Deep Value");
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct ChildFlat {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectFlat {
    pub field_a: String,
    #[xml(tree)]
    pub child_a: ChildFlat,
    #[xml(tree)]
    pub child_b: ChildFlat
}

#[test]
fn deserialize_flat() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <my_child attr="Attr A">
            <child_field_a>Other Value A</child_field_a>
        </my_child>
        <my_child attr="Attr B">
            <child_field_a>Other Value B</child_field_a>
        </my_child>
    </object>"#;

    let obj: XMLObjectFlat = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.child_a.child_field_a, "Other Value A");
    assert_eq!(obj.child_a.attribute, "Attr A");
    assert_eq!(obj.child_b.child_field_a, "Other Value B");
    assert_eq!(obj.child_b.attribute, "Attr B");
    Ok(())
}


#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct ChildA {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: ChildB
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct ChildB {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
    #[xml(tree)]
    pub inner: ChildC
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct ChildC {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    pub child_field_a: String,
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="child")]
struct ChildRoot {
    #[xml(tree)]
    pub inner: ChildA
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectTree {
    pub field_a: String,
    #[xml(tree)]
    pub child: ChildRoot
}

#[test]
fn deserialize_tree() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <child>
            <my_child attr="Outer Attr">
                <child_field_a>Outer Value</child_field_a>
                <my_child attr="Inner Attr">
                    <child_field_a>Inner Value</child_field_a>
                    <my_child  attr="Deep Attr">
                        <child_field_a>Deep Value</child_field_a>
                    </my_child>
                </my_child>
            </my_child>
        </child>
    </object>"#;
    let obj: XMLObjectTree = from_xml(&xml)?;
    let root = obj.child.inner;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(root.attribute, "Outer Attr");
    assert_eq!(root.child_field_a, "Outer Value");
    let inner = root.inner;
    assert_eq!(inner.attribute, "Inner Attr");
    assert_eq!(inner.child_field_a, "Inner Value");
    let deep = inner.inner;
    assert_eq!(deep.attribute, "Deep Attr");
    assert_eq!(deep.child_field_a, "Deep Value");
    Ok(())
}


#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child")]
struct ChildSibling {
    pub child_field_a: String
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectSibling {
    pub field_a: String,
    #[xml(tree)]
    pub siblings: Vec<ChildSibling>
}

#[test]
fn deserialize_sibling() -> Result<(), PError> {
    let xml = r#"
    <object>
        <my_child>
            <child_field_a>Other value A</child_field_a>
        </my_child>
        <my_child>
            <child_field_a>Other value B</child_field_a>
        </my_child>
        <fieldA>Some Text</fieldA>
    </object>"#;

    let obj: XMLObjectSibling = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.siblings[0].child_field_a, "Other value A");
    assert_eq!(obj.siblings[1].child_field_a, "Other value B");
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectSiblingPrimitive {
    pub field_a: String,
    #[xml(tree)]
    pub any: Vec<String>
}

//Should not work? (feature or bug?)
#[test]
fn deserialize_sibling_primitive() -> Result<(), PError> {
    let xml = r#"
    <object>
        <my_child>Other value A</my_child>
        <my_child>Other value B</my_child>
        <my_child_other>value other</my_child_other>
        <fieldA>Some Text</fieldA>
    </object>"#;

    let obj: XMLObjectSiblingPrimitive = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.any[0], "Other value A");
    assert_eq!(obj.any[1], "Other value B");
    Ok(())
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child_a")]
struct ChildSiblingA {
    pub child_field_a: String
}


#[derive(XmlDeserializable, Debug)]
#[xml(name="my_child_b")]
struct ChildSiblingB {
    pub child_field_a: String
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="object", case="Camel")]
struct XMLObjectSiblings {
    pub field_a: String,
    #[xml(tree)]
    pub siblings_a: Vec<ChildSiblingA>,
    #[xml(tree)]
    pub siblings_b: Vec<ChildSiblingB>
}

//Should work
#[test]
fn deserialize_siblings() -> Result<(), PError> {
    let xml = r#"
    <object>
        <fieldA>Some Text</fieldA>
        <my_child_a>
            <child_field_a>Other value AA</child_field_a>
        </my_child_a>
        <my_child_a>
            <child_field_a>Other value BA</child_field_a>
        </my_child_a>
        <my_child_b>
            <child_field_a>Other value AB</child_field_a>
        </my_child_b>
        <my_child_b>
            <child_field_a>Other value BB</child_field_a>
        </my_child_b>
    </object>"#;

    let obj: XMLObjectSiblings = from_xml(&xml)?;
    assert_eq!(obj.field_a, "Some Text");
    assert_eq!(obj.siblings_a[0].child_field_a, "Other value AA");
    assert_eq!(obj.siblings_a[1].child_field_a, "Other value BA");
    assert_eq!(obj.siblings_b[0].child_field_a, "Other value AB");
    assert_eq!(obj.siblings_b[1].child_field_a, "Other value BB");
    Ok(())
}
