use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate variant handler.
    pub fn generate_handle_variant(&self, select_variant: &Variant) -> TokenStream {
        let key = &select_variant.key;
        let variant_name = &select_variant.name;
        let enum_name = &self.enum_name;

        if select_variant.newtype {
            quote! {
                #key =>
                    ::compris::resolve::Resolve::resolve_with_errors(value, errors)?
                    .map(|value| #enum_name::#variant_name(value)),
            }
        } else {
            quote! {
                #key => ::std::option::Option::Some(#enum_name::#variant_name),
            }
        }
    }
}
