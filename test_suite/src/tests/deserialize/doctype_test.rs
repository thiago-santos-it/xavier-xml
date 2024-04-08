use xavier::{doctype, encode, from_obj, XmlSerializable};
use xavier::deserialize::error::PError;

#[derive(XmlSerializable)]
#[declaration]
#[dtd = "Note.dtd"]
#[xml(name="xml")]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = from_obj(&XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 });
    let (target, file) = doctype!(&xml)?;
    assert_eq!("xml", target);
    assert_eq!("Note.dtd", file);
    Ok(())
}