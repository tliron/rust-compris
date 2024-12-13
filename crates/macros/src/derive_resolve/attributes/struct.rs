use deluxe::*;

//
// StructAttribute
//

/// Struct-level attribute for `#[derive(Resolve)]`.
///
/// ```
/// #[derive(Resolve)]
/// #[resolve(...)] // this
/// struct MyStruct;
/// ```
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct StructAttribute {
    /// The concrete type of the context.
    ///
    /// If not present, will use a generic type parameter.
    pub context: Option<syn::Expr>,

    /// The concrete type of the error.
    ///
    /// If not present, will use a generic type parameter.
    pub error: Option<syn::Expr>,

    /// Whether to ignore keys that are unused by the fields.
    ///
    /// Cannot be used together with [FieldAttribute::other_keys](super::field_attribute::FieldAttribute::other_keys).
    #[deluxe(default)]
    pub ignore_other_keys: bool,
}
