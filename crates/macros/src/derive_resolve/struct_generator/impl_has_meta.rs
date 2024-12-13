use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl HasMeta`.
    pub fn generate_impl_has_meta(&self) -> syn::Result<TokenStream> {
        if let Some(citations_field_name) = &self.citations_field {
            let struct_name = &self.struct_name;
            let (impl_generics, struct_generics, where_clause) = self.struct_generics.split_for_impl();

            Ok(quote! {
                #[automatically_derived]
                impl #impl_generics ::compris::meta::HasMeta
                for #struct_name #struct_generics
                #where_clause
                {
                    fn get_meta(&self) -> ::std::option::Option<&::compris::meta::Meta>
                    {
                        match self.#citations_field_name.get(&::std::string::String::new()) {
                            ::std::option::Option::Some(citation) => match &citation.meta {
                                ::std::option::Option::Some(meta) => ::std::option::Option::Some(meta),
                                ::std::option::Option::None => ::std::option::Option::None,
                            },
                            ::std::option::Option::None => ::std::option::Option::None,
                        }
                    }

                    fn get_meta_mut(&mut self) -> ::std::option::Option<&mut ::compris::meta::Meta>
                    {
                        match self.#citations_field_name.get_mut(&::std::string::String::new()) {
                            ::std::option::Option::Some(citation) => match &mut citation.meta {
                                ::std::option::Option::Some(meta) => ::std::option::Option::Some(meta),
                                ::std::option::Option::None => ::std::option::Option::None,
                            },
                            ::std::option::Option::None => ::std::option::Option::None,
                        }
                    }
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
