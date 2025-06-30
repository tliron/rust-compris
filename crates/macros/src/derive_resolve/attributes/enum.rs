use deluxe::*;

//
// EnumAttribute
//

/// Enum-level attribute for `#[derive(Resolve)]`.
///
/// ```
/// #[derive(Resolve)]
/// #[resolve(...)] // this
/// enum MyEnum;
/// ```
#[derive(Default, ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct EnumAttribute {
    /// If set will use this existing generic parameter for the annotated field. Otherwise will insert
    /// a new parameter, "_AnnotatedT".
    #[deluxe(default)]
    pub annotated_parameter: Option<syn::Ident>,
}
