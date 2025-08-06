# Who is Xavier?

> This is a new lib so please report all bugs and help us!

Introducing Xavier: A Simplified XML Parsing Library **Inspired** by Serde. Why Xavier? Well... it starts with X, and it's the first name that came out of my mind, nothing else.

Xavier is a lightweight and versatile XML parsing library designed to streamline the process of handling XML data with ease and efficiency. 

While speed is a consideration in Xavier's design, it's important to emphasize that raw speed isn't its primary goal. Instead, Xavier prioritizes ease of use and ergonomic design, aiming to simplify XML parsing tasks within Rust applications without sacrificing reliability or developer experience.

**It must be used in relatively small xml because it stores all data in memory.**

> **Note 1:** UTF-16 is not supported yet. Hard work! PR's are welcome.

> **Note 2:** Our DOM implementation (WIP) aims to stick closely to the original specs, but achieving a perfect match is tough because of differences in how concepts are handled between the specs and Rust.

# Why not extend Serde?

Someone already did that, but I prefer to start from scratch. Besides, since Xavier focuses specifically on XML parsing, I believe it should be simpler and more tailored to that purpose.  

# Quick Start Guide

## Installation

Add Xavier to your `Cargo.toml`:

```toml
[dependencies]
xavier = "0.1.0"
```

## Basic Usage

### Serialization (Rust → XML)

```Rust
use xavier::{XmlSerializable, from_obj};

#[derive(XmlSerializable)]
struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        email: "john@example.com".to_string(),
    };
    
    let xml = from_obj(&person);
    println!("{}", xml);
}
```

Output:
```xml
<Person>
    <name>John Doe</name>
    <age>30</age>
    <email>john@example.com</email>
</Person>
```

### Deserialization (XML → Rust)

```Rust
use xavier::{XmlDeserializable, from_xml, PError};

#[derive(XmlDeserializable, Debug)]
struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

fn main() -> Result<(), PError> {
    let xml = r#"
    <Person>
        <name>John Doe</name>
        <age>30</age>
        <email>john@example.com</email>
    </Person>"#;
    
    let person: Person = from_xml(xml)?;
    println!("{:?}", person);
    Ok(())
}
```

### With Attributes and Custom Names

```Rust
use xavier::{XmlSerializable, XmlDeserializable, from_obj, from_xml, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="user", case="Camel")]
struct User {
    #[xml(attribute, name="id")]
    pub id: u64,
    #[xml(name="full_name")]
    pub name: String,
    pub email: String,
    pub active: bool,
}

fn main() -> Result<(), PError> {
    let user = User {
        id: 123,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        active: true,
    };
    
    // Serialize
    let xml = from_obj(&user);
    println!("Serialized: {}", xml);
    
    // Deserialize
    let parsed_user: User = from_xml(&xml)?;
    println!("Parsed: {:?}", parsed_user);
    
    Ok(())
}
```

Output:
```xml
<user id="123">
    <fullName>John Doe</fullName>
    <email>john@example.com</email>
    <active>true</active>
</user>
```

### Working with Collections

```Rust
use xavier::{XmlSerializable, XmlDeserializable, from_obj, from_xml, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="item")]
struct Item {
    pub name: String,
    pub quantity: u32,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="order")]
struct Order {
    #[xml(attribute, name="id")]
    pub id: String,
    pub customer: String,
    pub items: Vec<Item>,
}

fn main() -> Result<(), PError> {
    let order = Order {
        id: "ORD-001".to_string(),
        customer: "John Doe".to_string(),
        items: vec![
            Item { name: "Book".to_string(), quantity: 2 },
            Item { name: "Pen".to_string(), quantity: 5 },
        ],
    };
    
    let xml = from_obj(&order);
    println!("{}", xml);
    
    let parsed_order: Order = from_xml(&xml)?;
    println!("Order has {} items", parsed_order.items.len());
    
    Ok(())
}
```

### Error Handling

```Rust
use xavier::{from_xml, PError};

fn parse_safely(xml: &str) -> Result<Person, String> {
    match from_xml::<Person>(xml) {
        Ok(person) => Ok(person),
        Err(PError::ParseError(msg)) => {
            eprintln!("XML parsing error: {}", msg);
            Err("Invalid XML format".to_string())
        }
        Err(PError::Custom(msg)) => {
            eprintln!("Custom error: {}", msg);
            Err("Data validation failed".to_string())
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err("Unknown error occurred".to_string())
        }
    }
}
```

## Key Features at a Glance

- **Simple Macros**: Just add `#[derive(XmlSerializable)]` or `#[derive(XmlDeserializable)]`
- **Attribute Support**: Use `#[xml(attribute)]` for XML attributes
- **Custom Naming**: Control element names with `#[xml(name="...")]`
- **Case Conversion**: Automatic case conversion with `#[xml(case="Camel")]`
- **Namespaces**: Full XML namespace support
- **Collections**: Native support for `Vec<T>` and `HashMap<String, T>`
- **Optional Fields**: Use `Option<T>` for optional elements
- **Nested Structures**: Complex nested XML structures
- **Error Handling**: Comprehensive error types and messages

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
    #[xml(value)]  
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
    
    let instance: XMLObject = from_xml(&xml)?;
    assert_eq!(instance.some_string, "Some Content A");
    assert_eq!(instance.some_int, 0);
    assert_eq!(instance.some_float, 0.0);
// ... 
```

As you can see this is the same structure of tags as in serialize. Check out a lot of examples [HERE!](https://github.com/thiago-santos-it/xavier-xml/tree/main/test_suite/src/tests/deserialize)

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

### Errors

Xavier DOM (WIP) implementation use ```DOMException``` due to spec, but *"Xavier DeSer tiene un PError"* ```ʕ•ᴥ•ʔ```  

## Advanced Use Cases

### Configuration Files

Xavier is perfect for handling configuration files in XML format. Here's a comprehensive example:

```Rust
#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="config")]
struct AppConfig {
    #[xml(attribute, name="version")]
    pub version: String,
    #[xml(attribute, name="environment")]
    pub environment: String,
    
    #[xml(tree)]
    pub database: DatabaseConfig,
    #[xml(tree)]
    pub server: ServerConfig,
    #[xml(tree)]
    pub logging: LoggingConfig,
    #[xml(tree)]
    pub features: Vec<FeatureConfig>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="database")]
struct DatabaseConfig {
    #[xml(attribute, name="type")]
    pub db_type: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
    pub connection_pool: Option<u32>,
    pub timeout: Option<u64>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="server")]
struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub timeout: u64,
    #[xml(tree)]
    pub ssl: Option<SslConfig>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="ssl")]
struct SslConfig {
    pub enabled: bool,
    pub certificate_path: String,
    pub key_path: String,
    pub ca_path: Option<String>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="logging")]
struct LoggingConfig {
    pub level: String,
    pub file_path: Option<String>,
    pub max_file_size: Option<u64>,
    pub backup_count: Option<u32>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="feature")]
struct FeatureConfig {
    #[xml(attribute, name="name")]
    pub name: String,
    #[xml(attribute, name="enabled")]
    pub enabled: bool,
    pub description: Option<String>,
    #[xml(tree)]
    pub settings: HashMap<String, String>
}
```

Example XML:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<config version="1.0" environment="production">
    <database type="postgresql">
        <host>localhost</host>
        <port>5432</port>
        <name>myapp</name>
        <username>admin</username>
        <password>secret123</password>
        <connection_pool>20</connection_pool>
        <timeout>30</timeout>
    </database>
    
    <server>
        <host>0.0.0.0</host>
        <port>8080</port>
        <max_connections>1000</max_connections>
        <timeout>60</timeout>
        <ssl>
            <enabled>true</enabled>
            <certificate_path>/etc/ssl/cert.pem</certificate_path>
            <key_path>/etc/ssl/key.pem</key_path>
        </ssl>
    </server>
    
    <logging>
        <level>INFO</level>
        <file_path>/var/log/app.log</file_path>
        <max_file_size>10485760</max_file_size>
        <backup_count>5</backup_count>
    </logging>
    
    <feature name="api_v2" enabled="true">
        <description>New API version with improved performance</description>
        <settings>
            <rate_limit>1000</rate_limit>
            <cache_ttl>3600</cache_ttl>
        </settings>
    </feature>
    
    <feature name="beta_features" enabled="false">
        <description>Experimental features</description>
        <settings>
            <experimental_mode>true</experimental_mode>
        </settings>
    </feature>
</config>
```

### REST API Responses

Handling XML responses from REST APIs:

```Rust
#[derive(XmlDeserializable, Debug)]
#[xml(name="response")]
struct ApiResponse<T> {
    #[xml(attribute, name="status")]
    pub status: String,
    #[xml(attribute, name="code")]
    pub code: u16,
    pub message: String,
    #[xml(tree)]
    pub data: Option<T>,
    #[xml(tree)]
    pub errors: Option<Vec<ApiError>>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="error")]
struct ApiError {
    #[xml(attribute, name="code")]
    pub code: String,
    pub message: String,
    pub details: Option<String>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="user")]
struct User {
    #[xml(attribute, name="id")]
    pub id: u64,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub created_at: String,
    #[xml(tree)]
    pub profile: UserProfile
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="profile")]
struct UserProfile {
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    #[xml(tree)]
    pub preferences: HashMap<String, String>
}

// Usage example
fn handle_user_response(xml: &str) -> Result<ApiResponse<User>, PError> {
    let response: ApiResponse<User> = from_xml(xml)?;
    match response.status.as_str() {
        "success" => Ok(response),
        "error" => {
            eprintln!("API Error: {}", response.message);
            Ok(response)
        }
        _ => Err(PError::Custom("Unknown response status".to_string()))
    }
}
```

### RSS Feed Processing

Processing RSS feeds with Xavier:

```Rust
#[derive(XmlDeserializable, Debug)]
#[xml(name="rss")]
struct RssFeed {
    #[xml(attribute, name="version")]
    pub version: String,
    #[xml(tree)]
    pub channel: RssChannel
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="channel")]
struct RssChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub generator: Option<String>,
    #[xml(tree)]
    pub items: Vec<RssItem>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="item")]
struct RssItem {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: Option<String>,
    pub guid: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    #[xml(tree)]
    pub enclosures: Option<Vec<RssEnclosure>>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="enclosure")]
struct RssEnclosure {
    #[xml(attribute, name="url")]
    pub url: String,
    #[xml(attribute, name="type")]
    pub media_type: String,
    #[xml(attribute, name="length")]
    pub length: Option<u64>
}

// Usage example
fn parse_rss_feed(xml: &str) -> Result<RssFeed, PError> {
    let feed: RssFeed = from_xml(xml)?;
    println!("Feed: {}", feed.channel.title);
    println!("Items: {}", feed.channel.items.len());
    Ok(feed)
}
```

### SOAP Web Services

Handling SOAP requests and responses:

```Rust
#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="soap:Envelope")]
struct SoapEnvelope<T> {
    #[xml(xmlns)]
    pub namespaces: Namespaces,
    #[xml(tree)]
    pub header: Option<SoapHeader>,
    #[xml(tree)]
    pub body: SoapBody<T>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="soap:Header")]
struct SoapHeader {
    #[xml(tree)]
    pub authentication: Option<AuthHeader>,
    #[xml(tree)]
    pub custom_headers: HashMap<String, String>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="auth")]
struct AuthHeader {
    pub username: String,
    pub password: String,
    pub token: Option<String>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="soap:Body")]
struct SoapBody<T> {
    #[xml(tree)]
    pub content: T
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="GetUserRequest")]
struct GetUserRequest {
    pub user_id: u64,
    pub include_profile: Option<bool>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="GetUserResponse")]
struct GetUserResponse {
    pub user: User,
    pub status: String,
    pub timestamp: String
}

// Usage example
fn create_soap_request(user_id: u64) -> String {
    let namespaces = namespaces!(
        soap = "http://schemas.xmlsoap.org/soap/envelope/",
        api = "http://example.com/api/"
    );
    
    let request = SoapEnvelope {
        namespaces,
        header: Some(SoapHeader {
            authentication: Some(AuthHeader {
                username: "admin".to_string(),
                password: "secret".to_string(),
                token: None
            }),
            custom_headers: HashMap::new()
        }),
        body: SoapBody {
            content: GetUserRequest {
                user_id,
                include_profile: Some(true)
            }
        }
    };
    
    from_obj(&request)
}
```

### Data Import/Export

Handling data import/export scenarios:

```Rust
#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="data_export")]
struct DataExport {
    #[xml(attribute, name="version")]
    pub version: String,
    #[xml(attribute, name="exported_at")]
    pub exported_at: String,
    #[xml(tree)]
    pub metadata: ExportMetadata,
    #[xml(tree)]
    pub records: Vec<DataRecord>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="metadata")]
struct ExportMetadata {
    pub source_system: String,
    pub record_count: u64,
    pub schema_version: String,
    pub export_format: String,
    #[xml(tree)]
    pub custom_fields: HashMap<String, String>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="record")]
struct DataRecord {
    #[xml(attribute, name="id")]
    pub id: String,
    #[xml(attribute, name="type")]
    pub record_type: String,
    pub created_at: String,
    pub updated_at: String,
    #[xml(tree)]
    pub fields: HashMap<String, FieldValue>,
    #[xml(tree)]
    pub tags: Option<Vec<String>>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="field")]
struct FieldValue {
    #[xml(attribute, name="type")]
    pub value_type: String,
    #[xml(value)]
    pub value: String
}

// Usage example
fn export_data(records: Vec<DataRecord>) -> String {
    let metadata = ExportMetadata {
        source_system: "my_app".to_string(),
        record_count: records.len() as u64,
        schema_version: "1.0".to_string(),
        export_format: "xml".to_string(),
        custom_fields: HashMap::new()
    };
    
    let export = DataExport {
        version: "1.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        metadata,
        records
    };
    
    from_obj(&export)
}
```

### Nested Optional Fields

Handling complex nested structures with optional fields:

```Rust
#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="product")]
struct Product {
    #[xml(attribute, name="id")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category: String,
    #[xml(tree)]
    pub specifications: Option<ProductSpecs>,
    #[xml(tree)]
    pub variants: Option<Vec<ProductVariant>>,
    #[xml(tree)]
    pub reviews: Option<Vec<Review>>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="specifications")]
struct ProductSpecs {
    pub weight: Option<f64>,
    pub dimensions: Option<String>,
    pub material: Option<String>,
    pub warranty: Option<String>,
    #[xml(tree)]
    pub custom_specs: HashMap<String, String>
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="variant")]
struct ProductVariant {
    #[xml(attribute, name="sku")]
    pub sku: String,
    pub color: Option<String>,
    pub size: Option<String>,
    pub price_adjustment: f64,
    pub stock_quantity: u32
}

#[derive(XmlDeserializable, XmlSerializable, Debug)]
#[xml(name="review")]
struct Review {
    #[xml(attribute, name="rating")]
    pub rating: u8,
    pub author: String,
    pub title: Option<String>,
    pub content: String,
    pub date: String,
    pub verified_purchase: Option<bool>
}
```

### Error Handling Patterns

Best practices for error handling with Xavier:

```Rust
use std::collections::HashMap;
use xavier::{from_xml, PError, XmlDeserializable};

#[derive(XmlDeserializable, Debug)]
#[xml(name="result")]
struct ApiResult<T> {
    pub success: bool,
    pub message: String,
    #[xml(tree)]
    pub data: Option<T>,
    #[xml(tree)]
    pub errors: Option<Vec<ValidationError>>
}

#[derive(XmlDeserializable, Debug)]
#[xml(name="error")]
struct ValidationError {
    #[xml(attribute, name="field")]
    pub field: String,
    #[xml(attribute, name="code")]
    pub code: String,
    pub message: String
}

// Custom error handling
fn parse_with_validation<T: XmlDeserializable>(xml: &str) -> Result<T, Box<dyn std::error::Error>> {
    match from_xml::<T>(xml) {
        Ok(data) => Ok(data),
        Err(PError::Custom(msg)) => {
            eprintln!("Custom parsing error: {}", msg);
            Err(msg.into())
        }
        Err(PError::ParseError(msg)) => {
            eprintln!("XML parsing error: {}", msg);
            Err(msg.into())
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err("Unknown parsing error".into())
        }
    }
}

// Graceful degradation for optional fields
fn parse_with_fallbacks(xml: &str) -> Result<Product, PError> {
    let product: Product = from_xml(xml)?;
    
    // Provide defaults for missing optional fields
    let product = Product {
        description: product.description.or_else(|| Some("No description available".to_string())),
        specifications: product.specifications.or_else(|| Some(ProductSpecs {
            weight: None,
            dimensions: None,
            material: None,
            warranty: None,
            custom_specs: HashMap::new()
        })),
        ..product
    };
    
    Ok(product)
}
```

### Performance Considerations

Tips for optimal performance:

```Rust
// Use streaming for large XML files (when available)
// For now, Xavier loads everything into memory

// Pre-allocate collections when possible
#[derive(XmlDeserializable)]
struct LargeDataSet {
    pub items: Vec<DataItem>, // Xavier will handle this efficiently
}

// Use references where appropriate to avoid unnecessary cloning
#[derive(XmlDeserializable)]
struct OptimizedStruct {
    pub id: String,           // Owned - needed for struct
    pub reference: String,    // Owned - needed for struct
    // Avoid storing large strings multiple times
}

// Consider using Cow<str> for fields that might be borrowed
use std::borrow::Cow;

#[derive(XmlDeserializable)]
struct CowExample {
    pub name: Cow<'static, str>,  // Can be borrowed or owned
    pub description: Cow<'static, str>
}
```

### Integration with Web Frameworks

Example integration with Actix-web:

```Rust
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use xavier::{from_xml, from_obj, XmlDeserializable, XmlSerializable};

#[derive(XmlDeserializable, XmlSerializable)]
#[xml(name="user")]
struct User {
    pub id: u64,
    pub name: String,
    pub email: String
}

async fn create_user(user_xml: String) -> Result<HttpResponse, Error> {
    let user: User = from_xml(&user_xml)
        .map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;
    
    // Process user creation...
    
    let response_xml = from_obj(&user);
    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(response_xml))
}

async fn get_user_xml(path: web::Path<u64>) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    
    // Fetch user from database...
    let user = User {
        id: user_id,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string()
    };
    
    let xml = from_obj(&user);
    Ok(HttpResponse::Ok()
        .content_type("application/xml")
        .body(xml))
}
```

These advanced use cases demonstrate Xavier's flexibility and power for real-world applications. The library excels at handling complex XML structures while maintaining a clean and ergonomic API.

## Best Practices

### Struct Design

1. **Use Descriptive Names**: Choose clear, descriptive names for your structs and fields that match your XML schema.

```Rust
// Good
#[derive(XmlDeserializable)]
struct UserProfile {
    pub user_id: u64,
    pub email_address: String,
    pub account_status: String,
}

// Avoid
#[derive(XmlDeserializable)]
struct Data {
    pub id: u64,
    pub email: String,
    pub status: String,
}
```

2. **Group Related Fields**: Organize related fields together and use nested structs for complex data.

```Rust
#[derive(XmlDeserializable)]
struct Order {
    pub order_id: String,
    pub customer: CustomerInfo,
    pub items: Vec<OrderItem>,
    pub payment: PaymentDetails,
}

#[derive(XmlDeserializable)]
struct CustomerInfo {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}
```

3. **Use Appropriate Types**: Choose the right Rust types for your data.

```Rust
#[derive(XmlDeserializable)]
struct Product {
    pub id: u64,                    // Use u64 for IDs
    pub name: String,               // Use String for text
    pub price: f64,                 // Use f64 for prices
    pub in_stock: bool,             // Use bool for flags
    pub tags: Vec<String>,          // Use Vec for collections
    pub metadata: HashMap<String, String>, // Use HashMap for key-value pairs
}
```

### Attribute Usage

1. **Consistent Naming**: Use consistent naming conventions across your XML structure.

```Rust
#[derive(XmlDeserializable)]
#[xml(name="order", case="Camel")]
struct Order {
    #[xml(attribute, name="id")]
    pub order_id: String,
    #[xml(name="customer_name")]
    pub customer_name: String,
    #[xml(name="order_date")]
    pub order_date: String,
}
```

2. **Meaningful Attributes**: Use attributes for metadata and elements for content.

```Rust
// Good - ID as attribute, content as element
#[derive(XmlDeserializable)]
#[xml(name="product")]
struct Product {
    #[xml(attribute, name="id")]
    pub id: String,
    pub name: String,
    pub description: String,
}

// Avoid - mixing content in attributes
#[derive(XmlDeserializable)]
#[xml(name="product")]
struct Product {
    #[xml(attribute, name="id")]
    pub id: String,
    #[xml(attribute, name="name")]
    pub name: String,
    #[xml(attribute, name="description")]
    pub description: String,
}
```

### Error Handling

1. **Provide Context**: Include meaningful error messages and context.

```Rust
fn parse_user_data(xml: &str) -> Result<User, String> {
    from_xml::<User>(xml)
        .map_err(|e| format!("Failed to parse user data: {:?}", e))
}
```

2. **Graceful Degradation**: Handle missing optional fields gracefully.

```Rust
#[derive(XmlDeserializable)]
struct User {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,      // Optional field
    pub profile: Option<UserProfile>, // Optional nested struct
}

fn process_user(user: User) {
    let email = user.email.unwrap_or_else(|| "No email provided".to_string());
    let profile = user.profile.unwrap_or_else(|| UserProfile::default());
}
```

### Performance Tips

1. **Avoid Unnecessary Allocations**: Use references where possible.

```Rust
// When you need to store the data
#[derive(XmlDeserializable)]
struct Data {
    pub content: String,  // Owned data
}

// When you can borrow
#[derive(XmlDeserializable)]
struct DataRef<'a> {
    pub content: &'a str,  // Borrowed data (when supported)
}
```

2. **Pre-allocate Collections**: When you know the size, pre-allocate vectors.

```Rust
fn process_items(xml: &str) -> Result<Vec<Item>, PError> {
    let items: Vec<Item> = from_xml(xml)?;
    // Xavier handles this efficiently, but you can optimize further if needed
    Ok(items)
}
```

## Common Patterns

### Configuration Pattern

```Rust
#[derive(XmlDeserializable, XmlSerializable)]
#[xml(name="config")]
struct AppConfig {
    #[xml(attribute, name="version")]
    pub version: String,
    #[xml(tree)]
    pub database: DatabaseConfig,
    #[xml(tree)]
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = from_xml(&content)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let xml = from_obj(self);
        std::fs::write(path, xml)?;
        Ok(())
    }
}
```

### Builder Pattern

```Rust
#[derive(XmlDeserializable, XmlSerializable, Default)]
#[xml(name="request")]
struct ApiRequest {
    #[xml(attribute, name="method")]
    pub method: String,
    #[xml(attribute, name="version")]
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl ApiRequest {
    pub fn new(method: &str) -> Self {
        Self {
            method: method.to_string(),
            version: "1.0".to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }
    
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn with_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
    
    pub fn to_xml(&self) -> String {
        from_obj(self)
    }
}

// Usage
let request = ApiRequest::new("POST")
    .with_header("Content-Type", "application/json")
    .with_header("Authorization", "Bearer token")
    .with_body(r#"{"key": "value"}"#);
    
let xml = request.to_xml();
```

### Validation Pattern

```Rust
#[derive(XmlDeserializable)]
struct User {
    pub id: u64,
    pub email: String,
    pub age: u32,
}

impl User {
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.email.is_empty() {
            errors.push("Email cannot be empty".to_string());
        }
        
        if !self.email.contains('@') {
            errors.push("Email must contain @ symbol".to_string());
        }
        
        if self.age < 13 {
            errors.push("User must be at least 13 years old".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn parse_and_validate_user(xml: &str) -> Result<User, String> {
    let user: User = from_xml(xml)?;
    user.validate()
        .map_err(|errors| format!("Validation failed: {}", errors.join(", ")))?;
    Ok(user)
}
```

### Factory Pattern

```Rust
#[derive(XmlDeserializable)]
#[xml(name="message")]
struct Message {
    #[xml(attribute, name="type")]
    pub message_type: String,
    pub content: String,
}

trait MessageHandler {
    fn handle(&self, message: &Message) -> Result<(), String>;
}

struct EmailHandler;
struct SmsHandler;
struct PushHandler;

impl MessageHandler for EmailHandler {
    fn handle(&self, message: &Message) -> Result<(), String> {
        println!("Sending email: {}", message.content);
        Ok(())
    }
}

impl MessageHandler for SmsHandler {
    fn handle(&self, message: &Message) -> Result<(), String> {
        println!("Sending SMS: {}", message.content);
        Ok(())
    }
}

impl MessageHandler for PushHandler {
    fn handle(&self, message: &Message) -> Result<(), String> {
        println!("Sending push notification: {}", message.content);
        Ok(())
    }
}

struct MessageFactory;

impl MessageFactory {
    pub fn create_handler(message_type: &str) -> Box<dyn MessageHandler> {
        match message_type {
            "email" => Box::new(EmailHandler),
            "sms" => Box::new(SmsHandler),
            "push" => Box::new(PushHandler),
            _ => panic!("Unknown message type: {}", message_type),
        }
    }
}

fn process_message(xml: &str) -> Result<(), String> {
    let message: Message = from_xml(xml)?;
    let handler = MessageFactory::create_handler(&message.message_type);
    handler.handle(&message)
}
```

### Observer Pattern

```Rust
use std::collections::HashMap;

#[derive(XmlDeserializable)]
#[xml(name="event")]
struct Event {
    #[xml(attribute, name="type")]
    pub event_type: String,
    pub data: HashMap<String, String>,
    pub timestamp: String,
}

trait EventObserver {
    fn on_event(&self, event: &Event);
}

struct LoggingObserver;
struct MetricsObserver;
struct NotificationObserver;

impl EventObserver for LoggingObserver {
    fn on_event(&self, event: &Event) {
        println!("[LOG] Event: {} at {}", event.event_type, event.timestamp);
    }
}

impl EventObserver for MetricsObserver {
    fn on_event(&self, event: &Event) {
        println!("[METRICS] Recording event: {}", event.event_type);
    }
}

impl EventObserver for NotificationObserver {
    fn on_event(&self, event: &Event) {
        println!("[NOTIFICATION] Sending notification for: {}", event.event_type);
    }
}

struct EventProcessor {
    observers: Vec<Box<dyn EventObserver>>,
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
    
    pub fn add_observer(&mut self, observer: Box<dyn EventObserver>) {
        self.observers.push(observer);
    }
    
    pub fn process_event(&self, xml: &str) -> Result<(), PError> {
        let event: Event = from_xml(xml)?;
        
        for observer in &self.observers {
            observer.on_event(&event);
        }
        
        Ok(())
    }
}
```

These patterns and best practices will help you write more maintainable and efficient code with Xavier XML.

## Troubleshooting

### Common Issues and Solutions

#### 1. Compilation Errors

**Error: `expected String, found Option<String>`**

This usually happens when the macro generates code expecting an `Option<T>` but your struct field is `T`.

```Rust
// Problem
#[derive(XmlDeserializable)]
struct User {
    pub name: String,  // Field is String
}

// Solution: Make it optional if the field might not be present
#[derive(XmlDeserializable)]
struct User {
    pub name: Option<String>,  // Now it's optional
}

// Or ensure the field is always present in your XML
```

**Error: `cannot find derive macro XmlSerializable`**

Make sure you have the correct imports:

```Rust
use xavier::{XmlSerializable, XmlDeserializable, from_obj, from_xml, PError};
```

#### 2. Runtime Errors

**Error: `ParseError: Unexpected end of input`**

This usually means your XML is incomplete or malformed:

```Rust
// Problem - incomplete XML
let xml = "<user><name>John</name>";  // Missing closing tag

// Solution - ensure complete XML
let xml = "<user><name>John</name></user>";
```

**Error: `Custom: Field 'field_name' not found`**

This happens when the XML doesn't contain an expected field:

```Rust
// Problem - XML missing required field
let xml = "<user><name>John</name></user>";  // Missing 'email' field

#[derive(XmlDeserializable)]
struct User {
    pub name: String,
    pub email: String,  // Required field
}

// Solution 1: Make the field optional
#[derive(XmlDeserializable)]
struct User {
    pub name: String,
    pub email: Option<String>,  // Now optional
}

// Solution 2: Ensure the field is present in XML
let xml = "<user><name>John</name><email>john@example.com</email></user>";
```

#### 3. Attribute Issues

**Error: `Attribute 'attr_name' not found`**

Attributes must be explicitly marked with `#[xml(attribute)]`:

```Rust
// Problem
#[derive(XmlDeserializable)]
struct User {
    pub id: String,  // This expects an element, not an attribute
}

let xml = "<user id=\"123\"><name>John</name></user>";

// Solution
#[derive(XmlDeserializable)]
struct User {
    #[xml(attribute, name="id")]
    pub id: String,  // Now it expects an attribute
    pub name: String,
}
```

#### 4. Namespace Issues

**Error: `Namespace prefix not found`**

When working with namespaces, ensure proper setup:

```Rust
// Problem - missing namespace declaration
#[derive(XmlDeserializable)]
#[xml(ns="app")]
struct User {
    pub name: String,
}

let xml = "<app:user><name>John</name></app:user>";  // No xmlns declaration

// Solution - include xmlns attribute
#[derive(XmlDeserializable)]
#[xml(ns="app")]
struct User {
    #[xml(xmlns)]
    pub namespaces: Namespaces,
    pub name: String,
}

let namespaces = namespaces!(app = "http://example.com/app");
let user = User { namespaces, name: "John".to_string() };
```

#### 5. Collection Issues

**Error: `Expected collection element`**

Collections need proper XML structure:

```Rust
// Problem - incorrect XML structure for collections
let xml = "<users><user>John</user><user>Jane</user></users>";

#[derive(XmlDeserializable)]
struct Users {
    pub users: Vec<String>,  // Expects <users><users>John</users><users>Jane</users></users>
}

// Solution 1: Use proper XML structure
let xml = "<users><users>John</users><users>Jane</users></users>";

// Solution 2: Use a wrapper struct
#[derive(XmlDeserializable)]
#[xml(name="user")]
struct User {
    pub name: String,
}

#[derive(XmlDeserializable)]
struct Users {
    pub users: Vec<User>,
}

let xml = "<users><user><name>John</name></user><user><name>Jane</name></user></users>";
```

#### 6. Case Conversion Issues

**Error: `Element not found`**

Case conversion affects element names:

```Rust
// Problem - case mismatch
#[derive(XmlDeserializable)]
#[xml(case="Camel")]
struct User {
    pub first_name: String,  // Expects <firstName>
}

let xml = "<User><first_name>John</first_name></User>";  // Wrong case

// Solution - use correct case in XML
let xml = "<User><firstName>John</firstName></User>";
```

### Debugging Tips

#### 1. Enable Debug Output

Use `cargo expand` to see the generated code:

```bash
cargo install cargo-expand
cargo expand > expanded.rs
```

This will show you exactly what code the macro generates.

#### 2. Validate XML Structure

Use online XML validators to ensure your XML is well-formed:

```xml
<!-- Example of well-formed XML -->
<?xml version="1.0" encoding="UTF-8"?>
<users>
    <user id="1">
        <name>John Doe</name>
        <email>john@example.com</email>
    </user>
</users>
```

#### 3. Test Incrementally

Start with simple structures and build up complexity:

```Rust
// Start simple
#[derive(XmlDeserializable)]
struct SimpleUser {
    pub name: String,
}

// Then add attributes
#[derive(XmlDeserializable)]
struct UserWithId {
    #[xml(attribute, name="id")]
    pub id: String,
    pub name: String,
}

// Then add nested structures
#[derive(XmlDeserializable)]
struct UserWithProfile {
    pub name: String,
    #[xml(tree)]
    pub profile: UserProfile,
}
```

#### 4. Use Type Annotations

Explicitly specify types when deserializing:

```Rust
// Good - explicit type
let user: User = from_xml(xml)?;

// Avoid - type inference might fail
let user = from_xml(xml)?;
```

### Performance Issues

#### 1. Large XML Files

For large XML files, consider:

```Rust
// Use streaming when available (future feature)
// For now, Xavier loads everything into memory

// Consider breaking large files into smaller chunks
fn process_large_xml_in_chunks(xml: &str) -> Result<(), PError> {
    // Process in manageable chunks
    let chunks = split_xml_into_chunks(xml);
    for chunk in chunks {
        let data: DataChunk = from_xml(chunk)?;
        process_chunk(data)?;
    }
    Ok(())
}
```

#### 2. Memory Usage

Monitor memory usage with large collections:

```Rust
// Use appropriate collection types
#[derive(XmlDeserializable)]
struct OptimizedData {
    pub items: Vec<Item>,  // Good for sequential access
    pub lookup: HashMap<String, Item>,  // Good for key-based access
}
```

### Getting Help

If you encounter issues not covered here:

1. **Check the test suite**: Look at `test_suite/src/tests/` for examples
2. **Review error messages**: Xavier provides detailed error information
3. **Use cargo expand**: See the generated code to understand what's happening
4. **Report issues**: Create an issue on GitHub with:
   - Your Rust code
   - The XML you're trying to parse
   - The exact error message
   - Expected behavior

### Common Gotchas

1. **XML is case-sensitive**: `<Name>` and `<name>` are different elements
2. **Attributes vs Elements**: Use attributes for metadata, elements for content
3. **Optional fields**: Use `Option<T>` for fields that might not be present
4. **Namespaces**: Remember to declare namespaces when using them
5. **Collections**: Ensure proper XML structure for collections

# Backlog:

### Structs with Lifetime and Others

**Difficult: Easy**

The functions within TypeParser from ```deserialize::parser::complex::tokens::types``` handle type parsing in a statically structured manner, expecting elements to follow a predefined order. While effective for simpler Rust elements, this approach may require additional time and effort when dealing with more intricate Rust constructs. Nonetheless, the task is manageable, and with careful attention, we can effectively navigate through these complexities.

If necessary, you can modify the object creation process in ```constructors.rs``` or adjust the structure field assignments in ```setters/```.

### Implement DOM:

**Difficult: Medium**

(```branch feature/dom```)

Specs from https://www.w3.org/TR/REC-DOM-Level-1/level-one-core.html.

The DOM impl must be accessed as a Cargo feature called ```"dom"``` and can be used as follows: 

``` Rust
    //...
    let doc = to_dom(&xml);
    //...
    let xml = from_dom(&xml);
    //...        
```



  