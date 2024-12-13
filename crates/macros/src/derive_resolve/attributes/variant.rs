use deluxe::*;

//
// VariantAttribute
//

/// Variant-level attribute for `#[derive(Resolve)]`.
///
/// ```
/// #[derive(Resolve)]
/// enum MyEnum {
///   #[resolve(...)] // this
///   my_variant(Value)
/// }
/// ```
#[derive(Clone, ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct VariantAttribute {
    /// Map key used to select the variant.
    ///
    /// If not present, the key will be the variant name.
    pub key: Option<syn::Expr>,
}
