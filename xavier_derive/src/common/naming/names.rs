use syn::{DeriveInput, LitStr};
use convert_case::{Case, Casing};
use proc_macro2::Ident;

use crate::common::meta::MetaInfo;
use crate::common::naming::case::CaseFromStr;


pub struct XmlNames;

impl XmlNames {
    pub fn root(input: &DeriveInput, meta_info: Option<&MetaInfo>) -> String {
        let mut name = None;
        if let Some(meta_info) = meta_info {
            name = Some(XmlNames::compose_name(
                true,
                &meta_info.get_or("ns", "".to_string()),
                &meta_info.get_or("name", input.ident.to_string()),
                &meta_info.get_or("prefix", "".to_string()),
                &meta_info.get_or("suffix", "".to_string()),
                !meta_info.contains("no_prefix"),
                !meta_info.contains("no_suffix"),
                Case::value_from_str(&meta_info.get_or("case", "".to_string())),
            ));
        }
        name.unwrap_or(input.ident.to_string())
    }

    pub fn tag(field_name: &Ident, obj_meta_info: Option<&MetaInfo>, field_meta_info: Option<&MetaInfo>) -> LitStr {
        let empty = MetaInfo::empty();
        let obj_meta_info = obj_meta_info.unwrap_or(&empty);
        let field_meta_info = field_meta_info.unwrap_or(&empty);

        let ignore_case = &field_meta_info.get_or("ignore_case", "".to_string());
        let case = if ignore_case == "true" {
            None
        } else {
            Case::value_from_str(&obj_meta_info.get_or("case", "".to_string()))
        };

        let name = XmlNames::compose_name(
            false,
            &obj_meta_info.get_or("ns", "".to_string()),
            &field_meta_info.get_or("name", field_name.to_string()),
            &obj_meta_info.get_or("prefix", "".to_string()),
            &obj_meta_info.get_or("suffix", "".to_string()),
            true,
            true,
            case,
        );
        LitStr::new(&name, proc_macro2::Span::call_site())
    }


    pub fn attribute(attr_name: &Ident, obj_meta_info: Option<&MetaInfo>, attr_meta_info: &MetaInfo) -> LitStr {
        let empty = MetaInfo::empty();
        let obj_meta_info = obj_meta_info.unwrap_or(&empty);

        let ignore_case = &attr_meta_info.get_or("ignore_case", "".to_string());
        let case = if ignore_case == "true" {
            None
        } else {
            Case::value_from_str(&obj_meta_info.get_or("case", "".to_string()))
        };

        let name = XmlNames::compose_name(
            false,
            "",
            &attr_meta_info.get_or("name", attr_name.to_string()),
            &obj_meta_info.get_or("prefix", "".to_string()),
            &obj_meta_info.get_or("suffix", "".to_string()),
            attr_meta_info.contains("use_prefix"),
            attr_meta_info.contains("use_suffix"),
            case,
        );
        LitStr::new(&name, proc_macro2::Span::call_site())
    }

    fn compose_name(use_affix: bool, ns: &str, name: &str, prefix: &str, suffix: &str, use_suffix: bool, use_prefix: bool, case: Option<Case>) -> String {
        let namespace = if !ns.is_empty() {
            ns.to_string() + ":"
        } else {
            "".to_string()
        };

        let prefix = if use_affix {
            if use_prefix { prefix } else { "" }
        } else {
            prefix
        };

        let suffix = if use_affix {
            if use_suffix { suffix } else { "" }
        } else {
            suffix
        };

        let name = namespace + prefix + name + suffix;
        if let Some(case) = case {
            name.to_case(case)
        } else {
            name
        }
    }
}
