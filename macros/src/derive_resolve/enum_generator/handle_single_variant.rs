use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate single variant handler.
    pub fn generate_handle_single_variant(&self, variant: &Variant) -> TokenStream {
        let enum_name = &self.enum_name;
        let variant_name = &variant.name;

        quote! {
            if let ::compris::resolve::ResolveResult::Ok(::std::option::Option::Some(resolved)) =
                ::compris::resolve::Resolve::resolve_with_errors(self, &mut ::kutil_std::error::FailFastErrorRecipient)
            {
                return ::compris::resolve::ResolveResult::Ok(
                    ::std::option::Option::Some(#enum_name::#variant_name(resolved))
                );
            }
        }
    }
}
