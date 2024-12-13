use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate field handler.
    pub fn generate_handle_field(&self, field: &Field, key: &TokenStream) -> TokenStream {
        let handle_citation = if let Some(citations_field_name) = &self.citations_field {
            let quoted_field_name = field.name.to_string().to_token_stream();
            quote! {
                resolved.#citations_field_name.insert(
                    ::std::string::String::from(#quoted_field_name),
                    ::compris::cite::Citation::new_for(value, context, ancestor),
                );
            }
        } else {
            TokenStream::new()
        };

        let handle_null = Self::generate_handle_null(field, false);

        let handle_required = if field.attribute.required {
            quote! {
                else {
                    errors.give(
                        ::compris::resolve::WithCitationFor::with_citation_for(
                            ::compris::resolve::MissingRequiredKeyError::new(key), self, context, ancestor
                        )
                    )?;
                }
            }
        } else {
            TokenStream::new()
        };

        let field_name = &field.name;

        quote! {
            let key = #key;
            if let ::std::option::Option::Some(value) = self.into_get(key) {
                #handle_citation
                #handle_null
                if let ::std::option::Option::Some(value) =
                ::compris::resolve::Resolve::resolve_for(value, context, ancestor, errors)? {
                    resolved.#field_name = value;
                }
            }
            #handle_required
        }
    }
}
