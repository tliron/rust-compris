use super::super::{super::utils::*, attributes::*};

use {deluxe::*, proc_macro2::*, quote::*};

//
// EnumGenerator
//

/// Generator for `#[derive(Resolve)]` on enums.
#[derive(Default)]
pub struct EnumGenerator {
    /// Name of the enum for which we are generating.
    pub enum_name: TokenStream,

    /// The generics of the enum for which we are generating.
    pub enum_generics: syn::Generics,

    /// Enum-level attribute.
    pub enum_attribute: EnumAttribute,

    /// The variants that should be selected for.
    pub select_variants: Vec<Variant>,

    /// Will try to resolve to just this optional variant first as a "single" notation.
    pub single_variant: Option<Variant>,

    /// Human-readable variant key list.
    pub human_readable_key_list: String,
}

impl EnumGenerator {
    /// Generate.
    pub fn generate(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
        let generator = Self::new(input)?;
        let stream = generator.generate_impl_resolve();
        Ok(stream)
    }

    /// Constructor.
    pub fn new(input: &mut syn::DeriveInput) -> syn::Result<Self> {
        let mut generator = Self::default();

        generator.enum_name = input.ident.to_token_stream();
        generator.enum_generics = input.generics.clone();
        generator.enum_attribute = extract_attributes(input)?;

        match &mut input.data {
            syn::Data::Enum(data) => {
                for variant in data.variants.iter_mut() {
                    if attributes_have_ident(&variant.attrs, "resolve") {
                        match &variant.fields {
                            syn::Fields::Unnamed(fields) => {
                                if fields.unnamed.len() != 1 {
                                    return Err(syn::Error::new(
                                        variant.ident.span(),
                                        "`Resolve`: variants must have exactly 1 unnamed field",
                                    ));
                                }
                            }

                            syn::Fields::Named(_) => {
                                return Err(syn::Error::new(
                                    variant.ident.span(),
                                    "`Resolve`: variants with named fields are not supported",
                                ));
                            }

                            syn::Fields::Unit => {}
                        }

                        let variant_attribute: VariantAttribute = extract_attributes(variant)?;
                        let variant_name = &variant.ident;

                        let key = match &variant_attribute.key {
                            Some(key) => key.to_token_stream(),
                            None => (&variant_name.to_string()).to_token_stream(), // will add quotation marks
                        };

                        if variant_attribute.single {
                            if generator.single_variant.is_some() {
                                return Err(syn::Error::new(
                                    variant.ident.span(),
                                    "`resolve` attribute: only one variant may specify `single`",
                                ));
                            } else {
                                generator.single_variant = Some(Variant {
                                    key: key.clone(),
                                    name: variant_name.to_token_stream(),
                                    newtype: !variant.fields.is_empty(),
                                });
                            }
                        }

                        if !generator.human_readable_key_list.is_empty() {
                            generator.human_readable_key_list += " or ";
                        }
                        generator.human_readable_key_list += &key.to_string();

                        generator.select_variants.push(Variant {
                            key,
                            name: variant_name.to_token_stream(),
                            newtype: !variant.fields.is_empty(),
                        });
                    }
                }
            }

            _ => return Err(syn::Error::new(input.ident.span(), "`Resolve`: not an enum")),
        }

        Ok(generator)
    }
}

//
// Variant
//

/// Generator variant.
pub struct Variant {
    // Variant key.
    pub key: TokenStream,

    /// Variant name.
    pub name: TokenStream,

    /// Variant newtype.
    pub newtype: bool,
}
