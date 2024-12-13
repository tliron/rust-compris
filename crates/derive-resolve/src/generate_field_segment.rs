use super::field_attribute::*;

use {
    proc_macro2::TokenStream,
    quote::{quote, ToTokens},
    std::str::FromStr,
    syn::spanned::Spanned,
};

pub fn generate_field_segment(field: &mut syn::Field, struct_keys: &mut Vec<TokenStream>) -> syn::Result<TokenStream> {
    let attribute: FieldAttribute = deluxe::extract_attributes(field)?;

    match &field.ident {
        Some(name) => {
            // The attribute can set the key, e.g. #[resolve(key="mykey")]
            // otherwise we will use the field name
            let key = match &attribute.key {
                Some(key) => key.to_token_stream(),
                None => quoted_token_stream(&name.to_string())?,
            };

            struct_keys.push(key.clone());

            // Note we put #key in a variable in order to evaluate its expression just once
            let mut stream = quote! {
                let key = #key;
                if let Some(field) = self.get(key) {
                    if let Some(field) = ::compris::resolve::Resolve::resolve_for(field, ancestor, context, errors)? {
                        resolved.#name = field;
                    }
                }
            };

            // Required fields will report an error if key is missing
            if attribute.required {
                stream.extend(quote! {
                    else {
                        errors.give(
                            ::compris::resolve::WithCitationFor::with_citation_for(
                                ::compris::resolve::MissingRequiredKeyError::new(key), self, ancestor, context
                            )
                        )?;
                    }
                });
            }

            Ok(stream)
        }

        None => Err(syn::Error::new(field.span(), "unnamed field")),
    }
}

// Utils

fn quoted_token_stream(string: &str) -> syn::Result<TokenStream> {
    let string = format!("{:?}", string);
    Ok(TokenStream::from_str(&*string)?)
}
