use xavier::{declaration, encode, from_obj, XmlSerializable};
use xavier::deserialize::error::PError;

#[derive(XmlSerializable)]
#[declaration]
#[xml(name="object", case="Camel")]
struct XMLObject {
    pub field_a: String
}


#[test]
fn deserialize() -> Result<(), PError> {
    let xml = from_obj(&XMLObject { field_a: encode!("Teste Value") });
    let (version, encode, standalone) = declaration!(&xml)?;
    assert_eq!(version, "1.0");
    assert_eq!(encode.unwrap(), "UTF-8");
    assert_eq!(standalone.unwrap(), "no");
    Ok(())
}