use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate null handler.
    pub fn generate_handle_null(field: &Field, insert: bool) -> TokenStream {
        if field.attribute.ignore_null {
            quote! {
                if let ::compris::normal::Value::Null(_) = value {} else
            }
        } else {
            match &field.attribute.null {
                Some(null) => {
                    let field_name = &field.name;

                    if insert {
                        quote! {
                            if let ::compris::normal::Value::Null(_) = value
                                && let Some(key) = ::compris::resolve::Resolve::resolve_with_errors(key, errors)?
                            {
                                resolved.#field_name.insert(key, #null);
                            } else
                        }
                    } else {
                        quote! {
                            if let ::compris::normal::Value::Null(_) = value {
                                resolved.#field_name = #null;
                            } else
                        }
                    }
                }

                None => TokenStream::new(),
            }
        }
    }
}
