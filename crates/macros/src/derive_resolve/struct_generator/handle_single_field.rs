use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate single field handler.
    pub fn generate_handle_single_field(&self, field: &Field) -> TokenStream {
        let field_name = &field.name;

        quote! {
            if let ::compris::resolve::ResolveResult::Ok(::std::option::Option::Some(value)) =
            ::compris::resolve::Resolve::resolve_with_errors(self, &mut ::kutil_std::error::FailFastErrorRecipient) {
                resolved.#field_name = value;
                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(resolved)
                );
            }
        }
    }
}
