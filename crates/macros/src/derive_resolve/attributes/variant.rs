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

    /// Will try to resolve to just this variant first as a "single" notation.
    ///
    /// Can only be used on one variant.
    #[deluxe(default)]
    pub single: bool,
}
