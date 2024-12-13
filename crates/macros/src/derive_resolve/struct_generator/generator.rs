use super::super::{super::utils::*, attributes::*};

use {deluxe::*, proc_macro2::*, quote::*, syn::spanned::*};

//
// StructGenerator
//

/// Generator for `#[derive(Resolve)]` on structs.
#[derive(Default)]
pub struct StructGenerator {
    /// Name of the struct for which we are generating.
    pub struct_name: TokenStream,

    /// The generics of the struct for which we are generating.
    pub struct_generics: syn::Generics,

    /// Struct-level attribute.
    pub struct_attribute: StructAttribute,

    /// The fields that should be resolved.
    pub resolve_fields: Vec<(Field, TokenStream)>,

    /// The optional field used to store keys that are unused by the fields.
    pub other_keys_field: Option<Field>,

    /// The optional field used to store citations for all other fields.
    pub citations_field: Option<TokenStream>,

    /// All keys used by all resolved fields.
    pub declared_keys: Vec<TokenStream>,
}

impl StructGenerator {
    /// Generate.
    pub fn generate(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
        let generator = Self::new(input)?;

        let mut stream = generator.generate_impl_resolve();

        if generator.citations_field.is_some() {
            stream.extend(generator.generate_impl_citable());
            stream.extend(generator.generate_impl_citable_fields());
            stream.extend(generator.generate_impl_has_meta());
        }

        Ok(stream)
    }

    /// Constructor.
    pub fn new(input: &mut syn::DeriveInput) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.struct_name = input.ident.to_token_stream();
        generator.struct_generics = input.generics.clone();
        generator.struct_attribute = extract_attributes(input)?;

        match &mut input.data {
            syn::Data::Struct(data) => {
                for field in data.fields.iter_mut() {
                    if attributes_have_ident(&field.attrs, "resolve") {
                        let field_attribute: FieldAttribute = extract_attributes(field)?;

                        let field_name = match &field.ident {
                            Some(name) => name,
                            None => return Err(syn::Error::new(field.span(), "`resolve` attribute: unnamed field")),
                        };

                        if field_attribute.ignore_null && field_attribute.null.is_some() {
                            return Err(syn::Error::new(
                                field.span(),
                                "`resolve` attribute: can't specify both `ignore_null` and `null`",
                            ));
                        }

                        if field_attribute.other_keys {
                            if generator.other_keys_field.is_some() {
                                return Err(syn::Error::new(
                                    field.span(),
                                    "`resolve` attribute: only one field may specify `other_keys`",
                                ));
                            } else {
                                generator.other_keys_field = Some(Field {
                                    name: field_name.to_token_stream(),
                                    attribute: field_attribute.clone(),
                                });
                                continue;
                            }
                        }

                        if field_attribute.is_citations(field)? {
                            if generator.citations_field.is_some() {
                                return Err(syn::Error::new(
                                    field.span(),
                                    "`resolve` attribute: only one field may specify `citations`",
                                ));
                            } else {
                                generator.citations_field = Some(field_name.to_token_stream());
                                continue;
                            }
                        }

                        let key = match &field_attribute.key {
                            Some(key) => key.to_token_stream(),
                            None => (&field_name.to_string()).to_token_stream(), // will add quotation marks
                        };

                        generator.resolve_fields.push((
                            Field { name: field_name.to_token_stream(), attribute: field_attribute },
                            key.clone(),
                        ));

                        generator.declared_keys.push(key);
                    }
                }
            }

            _ => return Err(syn::Error::new(input.ident.span(), "`Resolve`: not a struct")),
        }

        if generator.struct_attribute.ignore_other_keys && generator.other_keys_field.is_some() {
            return Err(syn::Error::new(
                input.span(),
                "`resolve` attribute: can't specify both `ignore_other_keys` and `other_keys`",
            ));
        }

        Ok(generator)
    }
}

//
// Field
//

/// Generator field.
pub struct Field {
    /// Field name.
    pub name: TokenStream,

    /// Field attribute.
    pub attribute: FieldAttribute,
}
