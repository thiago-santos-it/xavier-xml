use quote::quote;
use syn::DeriveInput;
use crate::serialize::parser::complex::XmlComplexTag;
use crate::serialize::parser::simple::XmlSimpleTag;
use crate::serialize::parser::empty_tag::XmlEmptyTag;
use crate::serialize::parser::enumeration::XmlEnumValue;
use crate::serialize::parser::encoding::XmlEncoding;

pub struct XmlStream;

pub enum StreamType {
    Complex, Simple, Empty, Enum
}

impl XmlStream {
    pub(crate) fn stream(input: &DeriveInput, typed: StreamType) -> proc_macro2::TokenStream {
        let mut xml_stream = match typed {
            StreamType::Complex => XmlComplexTag::parse(input),
            StreamType::Simple => XmlSimpleTag::parse(input),
            StreamType::Empty => XmlEmptyTag::parse(input),
            StreamType::Enum => XmlEnumValue::parse(input)
        };

        let encoding = XmlEncoding::parse(input);
        let return_tokens = quote! {
            if #encoding == "UTF-16" {
                unimplemented!("UTF-16 is not supported yet");
            } else {
                xml
            }
        };
        xml_stream.extend(return_tokens);
        xml_stream
    }
}
