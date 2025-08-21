#[allow(unused_imports)]
use xavier::{from_obj, from_xml, from_xml_using_builder, PError, XmlDeserializable, XmlSerializable};


#[test]
fn isolated_test() -> Result<(), PError> {
    // let parsed: TestObject = from_xml_using_builder(&xml, TestObject::from_xml_dbg)?.unwrap();
    // let parsed: TestObject = from_xml(&xml)?;
    Ok(())
}

// impl TestObject {
//     pub fn from_xml_dbg(mut reader: &mut ::xavier::quick_xml::Reader<&[u8]>, start_event: Option<&::xavier::quick_xml::events::BytesStart>) -> Result<Option<Self>, xavier::PError> {
//
//     }
// }