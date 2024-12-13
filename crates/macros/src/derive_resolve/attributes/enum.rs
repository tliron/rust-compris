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
    /// The concrete type of the context.
    ///
    /// If not present, will use a generic type parameter.
    pub context: Option<syn::Expr>,

    /// The concrete type of the error.
    ///
    /// If not present, will use a generic type parameter.
    pub error: Option<syn::Expr>,
}
