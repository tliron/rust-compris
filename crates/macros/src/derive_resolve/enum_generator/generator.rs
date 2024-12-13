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
    pub select_variants: Vec<(Variant, TokenStream)>,
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
                        if matches!(variant.fields, syn::Fields::Named(_)) {
                            return Err(syn::Error::new(
                                variant.ident.span(),
                                "`Resolve`: variants with named fields are not supported",
                            ));
                        }

                        let variant_attribute: VariantAttribute = extract_attributes(variant)?;
                        let variant_name = &variant.ident;

                        let variant_type = match variant.fields.is_empty() {
                            true => None,
                            false => Some(variant.fields.to_token_stream()),
                        };

                        if let Some(variant_type) = &variant_type {
                            println!("+++++ {}", variant_type);
                        }

                        let key = match &variant_attribute.key {
                            Some(key) => key.to_token_stream(),
                            None => (&variant_name.to_string()).to_token_stream(), // will add quotation marks
                        };

                        generator.select_variants.push((
                            Variant {
                                name: variant_name.to_token_stream(),
                                type_: variant_type,
                                attribute: variant_attribute,
                            },
                            key.clone(),
                        ));
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
    /// Variant name.
    pub name: TokenStream,

    /// Variant type.
    pub type_: Option<TokenStream>,

    /// Variant attribute.
    pub attribute: VariantAttribute,
}
