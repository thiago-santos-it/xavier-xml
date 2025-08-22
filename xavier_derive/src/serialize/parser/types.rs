use syn::Type;

pub fn is_outer_option(ty: &Type) -> bool {
    if let Type::Path(typepath) = ty {
        if let Some(segment) = typepath.path.segments.first() {
            return segment.ident == "Option";
        }
    }
    false
}