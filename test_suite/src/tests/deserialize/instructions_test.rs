use xavier::{doctype, encode, from_obj, instructions, XmlSerializable};
use xavier::deserialize::error::PError;

#[derive(XmlSerializable)]
#[declaration]
#[pi(instr test="some")]
#[pi(instr test="some")]
#[xml(name="xml")]
struct XMLObject {
    pub some_string: String,
    #[pi(instr test="some")]
    pub some_int: i32,
    pub some_float: f32
}

#[test]
fn deserialize() -> Result<(), PError> {
    let xml = from_obj(&XMLObject { some_string: encode!("Some Text"), some_int: 0, some_float: 0.0 });
    instructions!(&xml, | tag, instruction, params | {
        assert_eq!("test=\"some\"", params);
        assert_eq!("instr", instruction);
    })?;
    Ok(())
}