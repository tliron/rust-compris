use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl Citable`.
    pub fn generate_impl_citable(&self) -> syn::Result<TokenStream> {
        if let Some(citations_field_name) = &self.citations_field {
            let struct_name = &self.struct_name;
            let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

            Ok(quote! {
                #[automatically_derived]
                impl #impl_generics ::compris::cite::Citable
                for #struct_name #struct_generics
                #where_clause
                {
                    fn get_citation(&self) -> ::std::option::Option<&::compris::cite::Citation>
                    {
                        self.#citations_field_name.get(&::std::string::String::new())
                    }

                    fn get_citation_mut(&mut self) -> ::std::option::Option<&mut ::compris::cite::Citation>
                    {
                        self.#citations_field_name.get_mut(&::std::string::String::new())
                    }
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}