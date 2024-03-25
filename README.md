# Who is Xavier?

Introducing Xavier: A Simplified XML Parsing Library **Inspired** by Serde. Why Xavier? Well... it starts with X, and it's the first name that came out of my mind, nothing else.

Xavier is a lightweight and versatile XML parsing library designed to streamline the process of handling XML data with ease and efficiency. 

While speed is a consideration in Xavier's design, it's important to emphasize that raw speed isn't its primary goal. Instead, Xavier prioritizes ease of use and ergonomic design, aiming to simplify XML parsing tasks within Rust applications without sacrificing reliability or developer experience.

**It must be used in relatively small xml because it stores all data in memory.**

# Why not extend Serde?

Someone already did that, but I prefer to start from scratch. Besides, since Xavier focuses specifically on XML parsing, I believe it should be simpler and more tailored to that purpose.  

# Examples

## Serialize 

### Starting simple:

This is the simplest example possible:

``` Rust
#[derive(XMLSerializable)]
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
#[derive(XMLSerializable)]
#[xml(name="object", case="Camel", prefix="xml_", suffix="Item", obj_use_suffix="false", obj_use_prefix="true")]
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
<xmlObject>
    <xmlJustStringItem>Some Content A</xmlJustStringItem>
    <xmlSomeIntItem>0</xmlSomeIntItem>
    <xmlSomeFloatItem>0.0</xmlSomeFloatItem>
</xmlObject>
```

> **Note 1:** Using camel config will produce to all elements use the same convention. 

> **Note 2:** All cases supported by convert_case crate can be used.

> **Note 3:** ignore_case can be used to ignore case in an element.

### Namespace

Working with namespaces:

``` Rust
#[derive(XMLSerializable)]
#[xml(ns="a", name="object", case="Camel")]
struct XMLObject {
    #[xmlns]
    pub namespaces: Namespaces,
    #[xml(name="just_string")]
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

// ... 
    let xmlns = namespaces!(
        a: "https://...", 
        b: "https://..."
    )
    XMLObject{ namespcaces: xmlns, ... }
    //...
    println!(from_obj(&instance));
    // ... 
```

Should produce:
``` xml
<a:object 
    xmlns:a = "https://..."
    xmlns:b = "https://...">
        <a:justString>Some Content A</justString>
        <a:someInt>0</someInt>
        <a:someFloat>0.0</someFloat>
</a:object>
```

> **Note:** ```#[xmlns]``` must be used only on root and only one time. 

### Attributes

Working with attributes:

``` Rust
#[derive(XMLSerializable)]
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
<a:object justString="Content A">
    <a:someInt>0</someInt>
    <a:someFloat>0.0</someFloat>
</a:object>
```

> Note: attr_use_suffix="false" or attr_use_prefix="true" can be used to avoid concatenations.

### Enum

Working with enums:
``` Rust
#[derive(XMLSerializable)]
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

#[derive(XMLSerializable)]
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
#[derive(XMLSerializable)]
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
#[derive(XMLSerializable)]
#[xml(name="object")]
struct XMLObject;

```

Should produce:
``` xml
<object></object>
```

> Not so useful as root element... but think about using it as flag field in a more complex context. 

### Complex Data

Composing structs like this: 

```Rust

#[derive(XMLSerializable)]
#[xml(name="my_child")]
struct Child {
    pub child_field_a: String,
}

#[derive(XMLSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(complex)]
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

### Structs as tags

Configuring nested struct as this:
```Rust

#[derive(XMLSerializable)]
#[xml(tag, name="child")]
struct Child {
    #[xml(attribute, name="attr")]
    pub attribute: String,
    #[xml(value)]
    pub value: String,
}

#[derive(XMLSerializable)]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String,
    #[xml(complex)]
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

### XML header

You can configure XML like this:
```Rust
#[derive(XMLSerializable)]
#[header(version="1.0" encoding="UTF-8" standaline = "no")]
#[xml(root), name="xml"]
struct XMLObject {
    //...
}
// or
#[header]
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

> Note: If not specified the default header is used with ```version="1.0" encoding="UTF-8" standaline = "no"```

### DTD

Using this:

```Rust
#[header]
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

### Convenience

#### CDATA

This:

``` Rust
  println!(xcdata!("Some text & others"));  
```

Prints this:
``` 
  <![CDATA[Some text &amp; others]]>
```

#### Text encoded

``` Rust
  println!(text!("Some text & others"));  
```

Prints this:
``` 
   Some text &amp; others
```

### Future:

- Flatten feature to remove the root tag or a complex element

### TODO

[ ] Tests
[ ] Encoding
[ ] Create root functions
[ ] Deserialize


