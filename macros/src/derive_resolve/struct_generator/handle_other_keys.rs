use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate other keys handler.
    pub fn generate_handle_other_keys(&self) -> TokenStream {
        // Optimization possibility: a key that was resolved successfully in the field segments
        // above is already known to be known, we don't need to retest it here

        let handle_other_keys = match &self.other_keys_field {
            Some(other_keys_field) => {
                let other_keys_field_name = &other_keys_field.name;
                let handle_null = Self::generate_handle_null(other_keys_field, true);

                quote! {
                    for (key, value) in &map.inner {
                        if !declared_keys.contains(key.into()) {
                            #handle_null
                            if let Some(key) = ::compris::resolve::Resolve::resolve_with_errors(key, errors)?
                                && let Some(value) = ::compris::resolve::Resolve::resolve_with_errors(value, errors)?
                            {
                                resolved.#other_keys_field_name.insert(key, value);
                            }
                        }
                    }
                }
            }

            None => quote! {
                for key in map.inner.keys() {
                    if !declared_keys.contains(key.into()) {
                        errors.give(
                            ::compris::annotate::Annotated::with_annotations_from(
                                ::compris::resolve::InvalidKeyError::new(key.clone()),
                                key,
                            )
                        )?;
                    }
                }
            },
        };

        let declared_keys = &self.declared_keys;

        quote! {
            if let ::compris::normal::Variant::Map(map) = self {
                let declared_keys = ::compris::normal_vec![ #(#declared_keys),* ];
                #handle_other_keys
            }
        }
    }
}
