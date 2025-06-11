use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate other keys handler.
    pub fn generate_handle_other_keys(&self) -> TokenStream {
        // Optimization possibility: a key that was resolved successfully in the field segments
        // above is aready known to be known, we don't need to retest it here
        let handle_other_keys = match self.other_keys_field {
            Some(ref field) => {
                let field_name = &field.name;
                let handle_null = Self::generate_handle_null(&field, true);

                quote! {
                    for (key, value) in &map.value {
                        if !declared_keys.contains(key.into()) {
                            #handle_null
                            if let Some(key) = ::compris::resolve::Resolve::resolve_for(key, context, ancestor, errors)? {
                                if let Some(value) = ::compris::resolve::Resolve::resolve_for(value, context, ancestor, errors)? {
                                    resolved.#field_name.insert(key, value);
                                }
                            }
                        }
                    }
                }
            }

            None => quote! {
                for key in map.value.keys() {
                    if !declared_keys.contains(key.into()) {
                        errors.give(
                            ::compris::resolve::WithCitationFor::with_citation_for(
                                ::compris::resolve::InvalidKeyError::new(key.clone()), key, context, ancestor
                            )
                        )?;
                    }
                }
            },
        };

        let declared_keys = &self.declared_keys;

        quote! {
            if let ::compris::normal::Value::Map(map) = self {
                let declared_keys = ::compris::normal_vec![ #(#declared_keys),* ];
                #handle_other_keys
            }
        }
    }
}
