use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deserialize::error::PError;

pub trait WrapWith<F, T> {
    fn wrap(self) -> Option<F>;
}

impl<T, F> WrapWith<F, Option<T>> for Option<T>
where
    F: From<T>,
{
    fn wrap(self) -> Option<F> {
        self.map(F::from)
    }
}

pub trait XmlDeserializable {
    fn from_xml(reader: &mut Reader<&[u8]>, event: Option<&BytesStart>) -> Result<Option<Self>, PError> where Self: Sized;
    fn inner_name() -> Option<String> { None }
}