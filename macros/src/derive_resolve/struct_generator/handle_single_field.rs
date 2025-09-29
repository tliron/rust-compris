use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate single field handler.
    pub fn generate_handle_single_field(&self) -> TokenStream {
        if let Some(single_field) = &self.single_field {
            let field_name = &single_field.name;

            // TODO: is there any way we can avoid cloning self?

            quote! {
                if let ::std::option::Option::Some(value) =
                    ::compris::resolve::Resolve::resolve_with_errors(self, errors)?
                {
                    resolved.#field_name = value;
                }

                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(resolved)
                );
            }
        } else {
            quote! {
                ::kutil::std::error::ErrorRecipient::give_error(
                    errors,
                    ::compris::normal::IncompatibleVariantTypeError::new_from(
                        &self,
                        &["map"],
                    ).into(),
                )?;

                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(resolved)
                );
            }
        }
    }
}
