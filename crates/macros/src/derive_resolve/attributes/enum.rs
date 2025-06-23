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
    /// If set will use this existing generic parameter for annotations. Otherwise will insert a
    /// new parameter, "_AnnotationsT".
    #[deluxe(default)]
    pub annotations_parameter: Option<syn::Ident>,
}
