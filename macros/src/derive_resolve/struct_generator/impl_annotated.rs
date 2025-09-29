use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl Annotated`.
    pub fn generate_impl_annotated(&self) -> syn::Result<TokenStream> {
        if let Some(annotations_field_name) = &self.annotations_field {
            let struct_name = &self.struct_name;
            let (impl_generics, type_generics, where_clause) = self.struct_generics.split_for_impl();

            Ok(quote! {
                #[automatically_derived]
                impl
                    #impl_generics ::compris::annotate::Annotated
                    for #struct_name #type_generics
                    #where_clause
                {
                    fn can_have_annotations() -> bool {
                        // We cannot test for this, so must assume true
                        true
                    }

                    fn annotations(&self) ->
                        ::std::option::Option<&::compris::annotate::Annotations>
                    {
                        self.#annotations_field_name.get("")
                    }

                    fn annotations_mut(&mut self) ->
                        ::std::option::Option<&mut ::compris::annotate::Annotations>
                    {
                        self.#annotations_field_name.get_mut("")
                    }
                }

                #[automatically_derived]
                impl
                    #impl_generics ::compris::annotate::AnnotatedStruct
                    for #struct_name #type_generics
                    #where_clause
                {
                    fn field_annotations(&self, name: &str) -> Option<&Annotations> {
                        self.#annotations_field_name.get(name)
                    }
                }
            })
        } else {
            Ok(Default::default())
        }
    }
}
