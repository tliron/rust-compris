//
// StructAttribute
//

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct StructAttribute {
    #[deluxe(default)]
    pub allow_unknown_keys: bool,

    pub error: Option<syn::Expr>,
}
