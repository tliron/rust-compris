use super::{generate_field_segment::*, struct_attribute::*};

use {proc_macro2::TokenStream, quote::quote, syn::spanned::Spanned};

pub fn generate_impl_resolve(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let mut allow_unknown_keys = false;
    if has_attribute(&input.attrs, "resolve") {
        let attribute: StructAttribute = deluxe::extract_attributes(input)?;
        allow_unknown_keys = attribute.allow_unknown_keys;
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
        // Optimization note: a key that was resolved successfully in the field segments
        // above is aready known to be valid, we don't need to test it here
        segments.push(quote! {
            const STRUCT_KEYS: &[&str] = &[#(#struct_keys),*];

            if let ::compris::Value::Map(map) = self {
                for key in map.value.keys() {
                    let mut valid = false;

                    if let ::compris::Value::Text(key_text) = key {
                        if STRUCT_KEYS.contains(&key_text.into()) {
                            valid = true;
                        }
                    }

                    if !valid {
                        error_reporter.report(
                            ::compris::ToLocatableError::with_location(
                                ::compris::resolve::UnknownKeyError::new(key.clone()), key, ancestor
                            )
                        )?;
                    }
                }
            }
        });
    }

    let me = &input.ident;

    Ok(quote! {
        impl
            <E: ::std::error::Error>
            Resolve<#me, E>
            for ::compris::Value
        {
            fn
                resolve_for
                <'a, ER: ::compris::resolve::ErrorReporter<::compris::resolve::ResolveError<E>>>
                (&'a self, error_reporter: &mut ER, mut ancestor: ::std::option::Option<&'a ::compris::Value>) ->
                ::std::result::Result<::std::option::Option<#me>, ::compris::resolve::ResolveError<E>>
            {
                if ancestor.is_none() {
                    ancestor = Some(self);
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
