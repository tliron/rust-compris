use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl CitableFields`.
    pub fn generate_impl_citable_fields(&self) -> syn::Result<TokenStream> {
        if let Some(citations_field_name) = &self.citations_field {
            let struct_name = &self.struct_name;
            let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

            Ok(quote! {
                #[automatically_derived]
                impl #impl_generics ::compris::cite::CitableFields
                for #struct_name #struct_generics
                #where_clause
                {
                    fn get_field_citation(&self, name: &str) ->
                    ::std::option::Option<&::compris::cite::Citation>
                    {
                        self.#citations_field_name.get(&::std::string::String::from(name))
                    }
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
