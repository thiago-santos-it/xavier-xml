use xavier::{from_xml, from_obj, XmlSerializable, XmlDeserializable, PError};

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

#[test]
fn documentation_basic_example() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        email: "john@example.com".to_string(),
    };
    
    let xml = from_obj(&person);
    println!("Generated XML: {}", xml);
    
    // Verificar se o XML contém os elementos esperados
    assert!(xml.contains("<Person>"));
    assert!(xml.contains("<name>John Doe</name>"));
    assert!(xml.contains("<age>30</age>"));
    assert!(xml.contains("<email>john@example.com</email>"));
    assert!(xml.contains("</Person>"));
    
    // Testar deserialização
    let parsed: Person = from_xml(&xml).unwrap();
    assert_eq!(parsed.name, "John Doe");
    assert_eq!(parsed.age, 30);
    assert_eq!(parsed.email, "john@example.com");
}

// Exemplo com atributos
#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="user")]
struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[test]
fn documentation_attributes_example() {
    let user = User {
        id: 1,
        name: "Jane Doe".to_string(),
        email: "jane@example.com".to_string(),
    };
    
    let xml = from_obj(&user);
    println!("User XML: {}", xml);
    
    // Verificar se usa o nome correto
    assert!(xml.contains("<user>"));
    assert!(xml.contains("</user>"));
    
    let parsed: User = from_xml(&xml).unwrap();
    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.name, "Jane Doe");
    assert_eq!(parsed.email, "jane@example.com");
}

// Exemplo com coleções
#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct BookCollection {
    pub books: Vec<Book>,
    pub owner: String,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct Book {
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[test]
fn documentation_collections_example() {
    let books = vec![
        Book {
            title: "The Rust Programming Language".to_string(),
            author: "Steve Klabnik".to_string(),
            year: 2018,
        },
        Book {
            title: "Programming Rust".to_string(),
            author: "Jim Blandy".to_string(),
            year: 2017,
        },
    ];
    
    let collection = BookCollection {
        books,
        owner: "John Doe".to_string(),
    };
    
    let xml = from_obj(&collection);
    println!("Collection XML: {}", xml);
    
    // Verificar se contém os livros
    assert!(xml.contains("The Rust Programming Language"));
    assert!(xml.contains("Programming Rust"));
    assert!(xml.contains("John Doe"));
    
    let parsed: BookCollection = from_xml(&xml).unwrap();
    assert_eq!(parsed.books.len(), 2);
    assert_eq!(parsed.owner, "John Doe");
    assert_eq!(parsed.books[0].title, "The Rust Programming Language");
    assert_eq!(parsed.books[1].title, "Programming Rust");
}

// Exemplo com namespaces
#[derive(XmlSerializable, XmlDeserializable, Debug)]
#[xml(name="config")]
struct Configuration {
    pub version: String,
    pub settings: Vec<Setting>,
}

#[derive(XmlSerializable, XmlDeserializable, Debug)]
struct Setting {
    pub key: String,
    pub value: String,
}

#[test]
fn documentation_namespaces_example() {
    let config = Configuration {
        version: "1.0".to_string(),
        settings: vec![
            Setting {
                key: "debug".to_string(),
                value: "true".to_string(),
            },
            Setting {
                key: "timeout".to_string(),
                value: "30".to_string(),
            },
        ],
    };
    
    let xml = from_obj(&config);
    println!("Config XML: {}", xml);
    
    // Verificar se usa o nome correto
    assert!(xml.contains("<config>"));
    assert!(xml.contains("</config>"));
    
    let parsed: Configuration = from_xml(&xml).unwrap();
    assert_eq!(parsed.version, "1.0");
    assert_eq!(parsed.settings.len(), 2);
    assert_eq!(parsed.settings[0].key, "debug");
    assert_eq!(parsed.settings[0].value, "true");
} 