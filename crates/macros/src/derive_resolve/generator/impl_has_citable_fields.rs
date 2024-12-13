use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl CitableFields`.
    pub fn generate_impl_citable_fields(&self) -> syn::Result<TokenStream> {
        if let Some(citations_field_name) = &self.citations_field {
            let struct_name = &self.struct_name;
            let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

            Ok(quote! {
                impl #impl_generics ::compris::resolve::CitableFields
                for #struct_name #struct_generics
                #where_clause
                {
                    fn get_field_citation(&self, name: &str) ->
                    ::std::option::Option<&::compris::citation::Citation>
                    {
                        self.#citations_field_name.get(&name.to_string())
                    }
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
