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
                    #impl_generics ::compris::annotation::Annotated
                    for #struct_name #type_generics
                    #where_clause
                {
                    fn is_annotated() -> bool {
                        // We cannot test for this, so must assume true
                        true
                    }

                    fn get_annotations(&self) ->
                        ::std::option::Option<&::compris::annotation::Annotations>
                    {
                        self.#annotations_field_name.get("")
                    }

                    fn get_annotations_mut(&mut self) ->
                        ::std::option::Option<&mut ::compris::annotation::Annotations>
                    {
                        self.#annotations_field_name.get_mut("")
                    }

                    fn set_annotations(&mut self, annotations: ::compris::annotation::Annotations) {
                        self.#annotations_field_name.insert("".into(), annotations);
                    }
                }

                #[automatically_derived]
                impl
                    #impl_generics ::compris::annotation::AnnotatedFields
                    for #struct_name #type_generics
                    #where_clause
                {
                    fn get_field_annotations(&self, name: &str) -> Option<&Annotations> {
                        self.#annotations_field_name.get(name)
                    }
                }
            })
        } else {
            Ok(TokenStream::new())
        }
    }
}
