use xavier::{doctype, encode, from_obj, instructions, XmlSerializable};
use xavier::deserialize::error::PError;

#[derive(XmlSerializable)]
#[declaration]
#[pi(instr test="some")]
#[xml(name="xml")]
struct XMLObject {
    pub some_string: String,
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = from_obj(&XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 });
    let _ = instructions!(&xml, None)?;
  //  assert_eq!("instr", instruction);
  //  assert_eq!("test", params[0].0);
  //  assert_eq!("some", params[0].1);
    Ok(())
}