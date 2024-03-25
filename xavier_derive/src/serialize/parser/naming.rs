use syn::{DeriveInput, LitStr};
use convert_case::{Case, Casing};
use proc_macro2::Ident;

use crate::serialize::parser::meta::MetaInfo;

trait CaseFromStr {
    fn value_from_str(input: &str) -> Option<Case>;
}
impl CaseFromStr for Case {
    fn value_from_str(input: &str) -> Option<Case> {
        match input {
            "Upper" => Some(Case::Upper),
            "Lower" => Some(Case::Lower),
            "Title" => Some(Case::Title),
            "Toggle" => Some(Case::Toggle),
            "Alternating" => Some(Case::Alternating),
            "Camel" => Some(Case::Camel),
            "Pascal" => Some(Case::Pascal),
            "UpperCamel" => Some(Case::UpperCamel),
            "Snake" => Some(Case::Snake),
            "UpperSnake" => Some(Case::UpperSnake),
            "ScreamingSnake" => Some(Case::ScreamingSnake),
            "Kebab" => Some(Case::Kebab),
            "Cobol" => Some(Case::Cobol),
            "Train" => Some(Case::Train),
            "Flat" => Some(Case::Flat),
            "UpperFlat" => Some(Case::UpperFlat),
            _ => None,
        }
    }
}

pub fn object_name(input: &DeriveInput, meta_info: Option<&MetaInfo>) -> String {
    let mut name = None;
    if let Some(meta_info) = meta_info {
        name = Some(compose_name(
            true,
            &meta_info.get_or("ns", "".to_string()),
            &meta_info.get_or("name", input.ident.to_string()),
            &meta_info.get_or("prefix", "".to_string()),
            &meta_info.get_or("suffix", "".to_string()),
            &meta_info.get_or("obj_use_prefix", "true".to_string()),
            &meta_info.get_or("obj_use_suffix", "true".to_string()),
            Case::value_from_str(&meta_info.get_or("case", "".to_string()))
        ));
    }
    name.unwrap_or(input.ident.to_string())
}

pub fn tag_name(field_name: &Ident, obj_meta_info: Option<&MetaInfo>, field_meta_info: Option<&MetaInfo>) -> LitStr {
    let empty = MetaInfo::empty();
    let obj_meta_info = obj_meta_info.unwrap_or(&empty);
    let field_meta_info = field_meta_info.unwrap_or(&empty);

    let ignore_case = &field_meta_info.get_or("ignore_case", "".to_string());
    let case = if ignore_case == "true" {
        None
    } else {
        Case::value_from_str(&obj_meta_info.get_or("case", "".to_string()))
    };

    let name = compose_name(
        false,
        &obj_meta_info.get_or("ns", "".to_string()),
        &field_meta_info.get_or("name", field_name.to_string()),
        &obj_meta_info.get_or("prefix", "".to_string()),
        &obj_meta_info.get_or("suffix", "".to_string()),
        "true",
        "true",
        case
    );
    LitStr::new(&name, proc_macro2::Span::call_site())
}


pub fn attribute_name(attr_name: &Ident, obj_meta_info: Option<&MetaInfo>, attr_meta_info: &MetaInfo) -> LitStr {
    let empty = MetaInfo::empty();
    let obj_meta_info = obj_meta_info.unwrap_or(&empty);

    let ignore_case = &attr_meta_info.get_or("ignore_case", "".to_string());
    let case = if ignore_case == "true" {
        None
    } else {
        Case::value_from_str(&obj_meta_info.get_or("case", "".to_string()))
    };

    let name = compose_name(
        false,
        "",
        &attr_meta_info.get_or("name", attr_name.to_string()),
        &obj_meta_info.get_or("prefix", "".to_string()),
        &obj_meta_info.get_or("suffix", "".to_string()),
        &obj_meta_info.get_or("attr_use_prefix", "false".to_string()),
        &obj_meta_info.get_or("attr_use_suffix", "false".to_string()),
        case
    );
    LitStr::new(&name, proc_macro2::Span::call_site())
}

fn compose_name(use_affix: bool, ns: &str, name: &str, prefix: &str, suffix: &str, use_suffix: &str, use_prefix: &str, case: Option<Case>) -> String {
    let namespace = if !ns.is_empty() {
        ns.to_string() + ":"
    } else {
        "".to_string()
    };

    let prefix = if use_affix {
      if use_prefix == "true" { prefix } else { "" }
    } else {
        prefix
    };

    let suffix = if use_affix {
        if use_suffix == "true" { suffix } else { "" }
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
