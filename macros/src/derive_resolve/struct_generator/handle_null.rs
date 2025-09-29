use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate null handler.
    pub fn generate_handle_null(field: &Field, insert: bool) -> TokenStream {
        if field.attribute.ignore_null {
            quote! {
                if value.is_null() {} else
            }
        } else {
            match &field.attribute.null {
                Some(null) => {
                    let field_name = &field.name;

                    if insert {
                        quote! {
                            if value.is_null() {
                                if let Some(key) = ::compris::resolve::Resolve::resolve_with_errors(key, errors)? {
                                    resolved.#field_name.insert(key, #null);
                                }
                            } else
                        }
                    } else {
                        quote! {
                            if value.is_null() {
                                resolved.#field_name = #null;
                            } else
                        }
                    }
                }

                None => Default::default(),
            }
        }
    }
}
