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
    /// Whether to ignore keys that are unused by the fields.
    ///
    /// Cannot be used together with
    /// [FieldAttribute::other_keys](super::field::FieldAttribute::other_keys).
    #[deluxe(default)]
    pub ignore_other_keys: bool,

    /// If set will use this existing generic parameter for annotations. Otherwise will insert a
    /// new parameter, "_AnnotatedT".
    #[deluxe(default)]
    pub annotated_parameter: Option<syn::Ident>,
}
