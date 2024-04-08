# Who is Xavier?

Introducing Xavier: A Simplified XML Parsing Library **Inspired** by Serde. Why Xavier? Well... it starts with X, and it's the first name that came out of my mind, nothing else.

Xavier is a lightweight and versatile XML parsing library designed to streamline the process of handling XML data with ease and efficiency. 

While speed is a consideration in Xavier's design, it's important to emphasize that raw speed isn't its primary goal. Instead, Xavier prioritizes ease of use and ergonomic design, aiming to simplify XML parsing tasks within Rust applications without sacrificing reliability or developer experience.

**It must be used in relatively small xml because it stores all data in memory.**

> **Note 1:** UTF-16 is not supported yet. Hard work! PR's are welcome.

> **Note 2:** Our DOM implementation aims to stick closely to the original specs, but achieving a perfect match is tough because of differences in how concepts are handled between the specs and Rust.

# Why not extend Serde?

Someone already did that, but I prefer to start from scratch. Besides, since Xavier focuses specifically on XML parsing, I believe it should be simpler and more tailored to that purpose.  

# Examples

## Serialize 

### Starting simple:

This is the simplest example possible:

``` Rust
#[derive(XmlSerializable)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    println!(from_obj(&instance));
// ... 
```

Should produce: 
``` xml
<XMLObject>
    <some_string>Some Content A</some_string>
    <some_int>0</some_int>
    <some_float>0.0</some_float>
</XMLObject>
```

### Names

Improving the names:

``` Rust
#[derive(XmlSerializable)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", no_suffix, no_prefix)]
struct XMLObject {
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    println!(from_obj(&instance));
// ... 
```

Should produce:
``` xml
<object>
    <xmlJustStringItem>Some Content A</xmlJustStringItem>
    <xmlSomeIntItem>0</xmlSomeIntItem>
    <xmlSomeFloatItem>0.0</xmlSomeFloatItem>
</object>
```

> **Note 1:** Using camel config will produce to all elements use the same convention. 

> **Note 2:** All cases supported by convert_case crate can be used, except Randoms.

> **Note 3:** ignore_case can be used to ignore case in an element.

### Namespace

Working with namespaces:

``` Rust
#[derive(XmlSerializable)]
#[xml(ns="xml", name="object", case="Camel")]
struct XMLObject {
    #[xml(xmlns)]
    pub namespaces: Namespaces,
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    let xmlns = namespaces!(xml = "http://www.w3.org/XML/1998/namespace", xhtml = "http://www.w3.org/1999/xhtml");
    XMLObject{ namespaces: xmlns, ... }
    //...
    println!(from_obj(&instance));
    // ... 
```

Should produce:
``` xml
<xml:object 
   xmlns:xml="http://www.w3.org/XML/1998/namespace" 
   xmlns:xhtml="http://www.w3.org/1999/xhtml">
        <xml:justString>Some Content A</justString>
        <xml:someInt>0</someInt>
        <xml:someFloat>0.0</someFloat>
</xml:object>
```

> **Note:** ```#[xml(xmlns)]``` must be used only on root and only one time. 

### Attributes

Working with attributes:

``` Rust
#[derive(XmlSerializable)]
#[xml(ns="a", name="object", case="Camel")]
struct XMLObject {
    #[xml(attribute, name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    println!(from_obj(&instance));
// ... 
```

Should produce:
``` xml
<a:xmlObject justString="Some Text">
    <a:someInt>0</a:someInt> 
    <a:someFloat>0</a:someFloat> 
</a:xmlObject>
```

> Note: use_suffix="false" or use_prefix="true" can be used to force suffix or prefix.

### Enum

Working with enums:
``` Rust
#[derive(XmlSerializable)]
enum CustomEnum {
    ValueA
}

// Many libs don't implement of infer any string value in this case, we are no exception.
impl Display for CustomEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CustomEnum::ValueA => { "Value A".to_string() },
        };
        write!(f, "{}", str)
    }
}

#[derive(XmlSerializable)]
#[xml(name="object")]
struct XMLObject {
    pub enum_field: CustomEnum,
}

// ...
     println!(from_obj(&instance));
// ...
```

Should produce:
``` xml
<object>
    <enum_field>ValueA</enum_field>
</object>
```

### Unnamed Struct

Using a unit struct like this:

``` Rust
#[derive(XmlSerializable)]
#[xml(ns="a", name="object")]
pub struct XMLObject(String);
```

Should produce:

``` xml
<a:object>Some Text</a:object>
```

> Note: More than one attribute in this case is not supported and will produce compile error.

### Unit Struct

Using a unit struct like this: 

``` Rust
#[derive(XmlSerializable)]
#[xml(name="object")]
struct XMLObject;

```

Should produce:
``` xml
<object></object>
```

> Not so useful as root element... but think about using it as flag field in a more tree context. 

### Trees

Composing structs like this: 

```Rust
#[derive(XmlSerializable)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)] //Same as #[xml(flatten)] 
    pub child: Child
}
```
Should produce:

``` xml
<object>
    <fieldA>Some value</fieldA>
    <my_child>
        <child_field_a>Other value</child_field_a>
    </my_child>    
</object>
```
> Note: Case has the scope of the element. Same for namespaces.


### Collections

Composing structs like this:

```Rust
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
```
Should produce:

``` xml
<object>
	<fieldA>Some Text</fieldA>
	<children>
		<my_child>
			<child_field_a>Child A</child_field_a>
		</my_child>
		<my_child>
			<child_field_a>Child B</child_field_a>
		</my_child>
	</children>
</object>
```

> Note: ```HashMap<String, T: XmlSerializable>``` is also supported but with no naming effect.

### Structs as tags

Configuring nested struct as this:
```Rust
#[derive(XmlSerializable)]
#[xml(tag, name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    #[xml(value)] //Same as #[xml(flatten)] 
    pub value: String,
}

#[derive(XmlSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(tree)]
    pub child: Child
}
```

Should produce:

``` xml
<object>
    <fieldA>Some value</fieldA>
    <child attr="Attr Value">Other value</child>    
</object>
```
> Note 1: You can have as many attribute as you want, but just one value!
> Note 2: If not specified the default behaviour for a field is attribute, with empty value. 

### XML declaration

You can configure XML like this:
```Rust
#[derive(XmlSerializable)]
#[declaration(version="1.0" encoding="UTF-8" standaline = "no")]
#[xml(name="xml")]
struct XMLObject {
    //...
}
// or
#[derive(XmlSerializable)]
#[declaration]
#[xml(name="xml")]
struct XMLObject {
    //...
}
```

Should produce:
```xml
<?xml version = "1.0" encoding = "UTF-8" standalone = "no" ?>
<xml>
    ...
</xml>
```

> Note: If not specified the default declaration is used with ```version="1.0" encoding="UTF-8" standaline = "no"```

### DTD

Using this:

```Rust
#[derive(XmlSerializable)]
#[declaration]
#[dtd = "Note.dtd"]
#[xml(name="xml")]
struct XMLObject {
    //...
}
```

Should produce:
```xml
<?xml version = "1.0" encoding = "UTF-8" standalone = "no" ?>
<!DOCTYPE xml SYSTEM "Note.dtd">
<xml>
    ...
</xml>
```

> Note 1: Inline DTD is not supported at the moment. However, I'm open to exploring alternative methods. Pull requests are welcome and appreciated.
> Note 2: XML validation is out of scope of this project.


### PI (processing instruction)

Using this:

```Rust
#[derive(XmlSerializable)]
#[declaration]
#[pi(something key="value" flag)]
#[xml(name="xml")]
struct XMLObject {
    //...
}
```

Should produce:
```xml
<?xml version = "1.0" encoding = "UTF-8" standalone = "no" ?>
<?something key="value" flag?>
<xml>
    ...
</xml>
```

### Convenience

#### CDATA

This:

``` Rust
  println!(cdata!("Some text & others"));  
```

Prints this:
``` 
  <![CDATA[Some text & others]]>
```

#### Text encoded

``` Rust
  println!(encode!("Some text & others"));  
```

Prints this:
``` 
   Some text &amp; others
```

#### Comment

This:

``` Rust
  println!(comment!("Some text & others"));  
```

Prints this:
``` 
  <!--Some text & others-->
```

## Deserialize

### Starting simple:

This is the simplest example possible:

``` Rust
#[derive(XmlDeserializable)]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    let xml = r#"
    <XMLObject>
        <some_string>Some Content A</some_string>
        <some_int>0</some_int>
        <some_float>0.0</some_float>
    </XMLObject>"#
    
    let instance = to_obj(&xml)
    assert_eq!(instance.some_string, "Some Content A");
    assert_eq!(instance.some_int, 0);
    assert_eq!(instance.some_float, 0.0);
// ... 
```

As you can see this is the same structure of tags as in serialize. 

### Names, Attributes, Enum, Unnamed Struct, Unit Struct, Trees, Collections and Structs as tags

Works exactly like serialize but in opposite direction. Same tags! 😊

### Convenience

#### XML declaration

Declarations can be parsed using this macro!

```Rust
    let (version, encoding, standalone) = declaration!(&xml);
```

#### DTD

DTD's can be parsed using this macro!

```Rust
    let (target, file) = dtd!(&xml);
```

#### PI (processing instruction)

PI's can be parsed using this macro! 

```Rust
    instructions!(&xml, | tag, instruction, params | {
        // DO something related with the instruction itself
    });
```

#### Text decode

``` Rust
  println!(decode!("Some text &amp; others"));  
```

Prints this:
``` 
   Some text & others
```

### Namespaces

Will be available as a normal tag attribute.



# Backlog:

1 -  Implement DOM (branch feature/dom)

### Description:

Specs from https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html.

The DOM impl must be accessed as a Cargo feature called ```"dom"``` and can be used as follows: 

``` Rust
    //...
    let doc = to_dom(&xml);
    //...
    let xml = from_dom(&xml);
    //...        
```