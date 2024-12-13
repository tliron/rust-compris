use super::{generate_field_segment::*, struct_attribute::*};

use {proc_macro2::TokenStream, quote::quote, syn::spanned::Spanned};

pub fn generate_impl_resolve(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let mut allow_unknown_keys = false;
    let mut error = None;
    if has_attribute(&input.attrs, "resolve") {
        let attribute: StructAttribute = deluxe::extract_attributes(input)?;
        allow_unknown_keys = attribute.allow_unknown_keys;
        error = attribute.error;
    };

    let data = {
        if let syn::Data::Struct(data) = &mut input.data {
            data
        } else {
            return Err(syn::Error::new(input.span(), "not a struct"));
        }
    };

    let mut segments = Vec::new();
    let mut struct_keys = Vec::new();

    for field in data.fields.iter_mut() {
        if has_attribute(&field.attrs, "resolve") {
            segments.push(generate_field_segment(field, &mut struct_keys)?);
        }
    }

    if !allow_unknown_keys {
        // Optimization possibility: a key that was resolved successfully in the field segments
        // above is aready known to be valid, we don't need to test it here
        segments.push(quote! {
            const STRUCT_KEYS: &[&str] = &[#(#struct_keys),*];

            if let ::compris::normal::Value::Map(map) = self {
                for key in map.value.keys() {
                    let mut valid = false;

                    if let ::compris::normal::Value::Text(key_text) = key {
                        if STRUCT_KEYS.contains(&key_text.into()) {
                            valid = true;
                        }
                    }

                    if !valid {
                        errors.give(
                            ::compris::resolve::WithCitationFor::with_citation_for(
                                ::compris::resolve::UnknownKeyError::new(key.clone()), key, ancestor, context
                            )
                        )?;
                    }
                }
            }
        });
    }

    let me = &input.ident;

    let (error, generics) = if let Some(error) = error {
        // Concrete error type
        (quote! { #error }, TokenStream::new())
    } else {
        // Generic error type
        (quote! { E }, quote! { <E: ::compris::resolve::ResolveError> })
    };

    Ok(quote! {
        impl
            #generics
            Resolve<#me, #error>
            for ::compris::normal::Value
        {
            fn
                resolve_for
                <'a, ER: ::compris::resolve::ErrorRecipient<#error>>
                (
                    &'a self,
                    mut ancestor: ::std::option::Option<&'a ::compris::normal::Value>,
                    context: ::std::option::Option<&'a ::compris::resolve::ResolveContext>,
                    errors: &mut ER
                )
                -> ::compris::resolve::ResolveResult<#me, #error>
            {
                if ancestor.is_none() {
                    ancestor = Some(self)
                }

                let mut resolved: #me = ::std::default::Default::default();
                #(#segments)*
                Ok(Some(resolved))
            }
        }
    })
}

// Utils

fn has_attribute(attributes: &Vec<syn::Attribute>, name: &str) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident(name) {
            return true;
        }
    }
    false
}
