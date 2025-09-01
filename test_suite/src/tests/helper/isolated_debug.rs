#[allow(unused_imports)]
use xavier::{from_obj, from_xml, from_xml_using_builder, PError, XmlDeserializable, XmlSerializable};

#[derive(XmlDeserializable, Debug, PartialEq)]
struct TestStruct {

}

#[test]
fn test_dbg() -> Result<(), PError> {

    //let result = from_xml_using_builder(&xml, TestStruct::from_xml_dbg);

    Ok(())
}

 impl TestStruct {
     #[allow(dead_code, unused_mut, unused_variables)]
     pub fn from_xml_dbg(mut reader: &mut ::xavier::quick_xml::Reader<&[u8]>, start_event: Option<&::xavier::quick_xml::events::BytesStart>) -> Result<Option<Self>, xavier::PError> {
        Err(PError::new("PError"))
     }
 }