use convert_case::Case;

pub trait CaseFromStr {
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
