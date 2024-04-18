use syn::DeriveInput;
use proc_macro2::TokenStream;

use crate::deserialize::parser::complex::stream::XmlComplex;
use crate::deserialize::parser::empty::XmlEmptyTag;
use crate::deserialize::parser::enums::XmlEnum;
use crate::deserialize::parser::simple::XmlSimpleTag;

pub struct XmlDeStream;

pub enum DeStreamType {
    Complex, Simple, Empty, Enum
}

impl XmlDeStream {
    pub fn stream(input: &DeriveInput, typed: DeStreamType) -> TokenStream {
        match typed {
            DeStreamType::Complex => XmlComplex::parse(input),
            DeStreamType::Simple => XmlSimpleTag::parse(input),
            DeStreamType::Empty => XmlEmptyTag::parse(input),
            DeStreamType::Enum => XmlEnum::parse(input)
        }
    }
}
