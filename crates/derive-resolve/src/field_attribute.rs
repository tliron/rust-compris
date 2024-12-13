//
// ResolveAttribute
//

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct FieldAttribute {
    #[deluxe(default)]
    pub required: bool,

    pub key: Option<syn::Expr>,
}
