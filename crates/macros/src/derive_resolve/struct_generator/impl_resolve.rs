use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let annotations_parameter = self.annotations_parameter();

        let mut segments = Vec::new();

        // Self annotations as empty-string field
        if let Some(annotations_field_name) = &self.annotations_field {
            segments.push(quote! {
                if #annotations_parameter::is_annotated()
                    && let ::std::option::Option::Some(annotations) = self.get_annotations()
                {
                    resolved.#annotations_field_name.insert(
                        "".into(),
                        annotations.clone(),
                    );
                }
            });
        }

        if let Some(single_field) = &self.single_field {
            segments.push(self.generate_handle_single_field(single_field));
        }

        for (resolve_field, key) in &self.resolve_fields {
            segments.push(self.generate_handle_field(resolve_field, key, &annotations_parameter));
        }

        if !self.struct_attribute.ignore_other_keys {
            segments.push(self.generate_handle_other_keys());
        }

        let struct_name = &self.struct_name;
        let (impl_generics, type_generics, where_clause) = self.generics(&annotations_parameter);

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                Resolve<#struct_name #type_generics, #annotations_parameter>
                for ::compris::normal::Value<#annotations_parameter>
                #where_clause
            {
                fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) ->
                    ::compris::resolve::ResolveResult<#struct_name #type_generics, #annotations_parameter>
                    where ErrorRecipientT:
                        ::kutil_std::error::ErrorRecipient<::compris::resolve::ResolveError<#annotations_parameter>>
                {
                    let mut resolved: #struct_name #type_generics = ::std::default::Default::default();

                    #(#segments)*

                    ::compris::resolve::ResolveResult::Ok(
                        ::std::option::Option::Some(resolved)
                    )
                }
            }
        }
    }
}
