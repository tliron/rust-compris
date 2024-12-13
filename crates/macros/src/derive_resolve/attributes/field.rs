use {deluxe::*, syn::spanned::*};

//
// FieldAttribute
//

/// Field-level attribute for `#[derive(Resolve)]`.
///
/// ```
/// #[derive(Resolve)]
/// struct MyStruct {
///   #[resolve(...)] // this
///   my_field: Value
/// }
/// ```
#[derive(Clone, ExtractAttributes)]
#[deluxe(attributes(resolve))]
pub struct FieldAttribute {
    /// Key to retrieve value from map for resolving the field.
    ///
    /// If not present, the key will be the field name.
    pub key: Option<syn::Expr>,

    /// Whether this field must be resolved.
    ///
    /// Will report an error if the key is not in the map.
    #[deluxe(default)]
    pub required: bool,

    /// Whether to ignore null values instead of reporting an error.
    ///
    /// The field will simply stay at its default value.
    ///
    /// Cannot be used together with [FieldAttribute::null].
    #[deluxe(default)]
    pub ignore_null: bool,

    /// Value to use when resolving a null value.
    ///
    /// Cannot be used together with [FieldAttribute::ignore_null].
    pub null: Option<syn::Expr>,

    /// Use this field to store keys that are unused by the fields.
    ///
    /// Can only be used on one field.
    ///
    /// Cannot be used together with [StructAttribute::ignore_other_keys](super::r#struct::StructAttribute::ignore_other_keys).
    #[deluxe(default)]
    pub other_keys: bool,

    /// Use this field to store citations for all other fields.
    ///
    /// Can only be used on one field.
    #[deluxe(default)]
    pub citations: bool,
}

impl FieldAttribute {
    /// Whether this is a valid [FieldAttribute::citations] field.
    pub fn is_citations(&self, field: &syn::Field) -> syn::Result<bool> {
        if self.citations {
            if self.key.is_some() || self.required || self.ignore_null || self.null.is_some() || self.other_keys {
                return Err(syn::Error::new(
                    field.span(),
                    "`resolve` attribute: can't specify other flags with `citations`",
                ));
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
