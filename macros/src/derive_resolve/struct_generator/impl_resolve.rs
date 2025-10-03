use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let annotated_parameter = self.annotated_parameter();

        let mut segments = Vec::default();

        // Self annotations as empty-string field
        if let Some(annotations_field_name) = &self.annotations_field {
            segments.push(quote! {
                if #annotated_parameter::can_have_annotations()
                    && let ::std::option::Option::Some(annotations) =
                    ::compris::annotate::Annotated::annotations(&map)
                {
                    resolved.#annotations_field_name.insert(
                        "".into(),
                        annotations.clone(),
                    );
                }
            });
        }

        let single_field = self.generate_handle_single_field();

        for (resolve_field, key) in &self.resolve_fields {
            segments.push(self.generate_handle_field(resolve_field, key, &annotated_parameter));
        }

        if !self.struct_attribute.ignore_other_keys {
            segments.push(self.generate_handle_other_keys());
        }

        let struct_name = &self.struct_name;
        let (impl_generics, type_generics, where_clause) = self.generics(&annotated_parameter);

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                Resolve<#struct_name #type_generics, #annotated_parameter>
                for ::compris::normal::Variant<#annotated_parameter>
                #where_clause
            {
                fn resolve_with_errors<ErrorRecipientT>(mut self, errors: &mut ErrorRecipientT) ->
                    ::compris::resolve::ResolveResult<#struct_name #type_generics, #annotated_parameter>
                    where ErrorRecipientT:
                        ::kutil::std::error::ErrorRecipient<::compris::resolve::ResolveError<#annotated_parameter>>
                {
                    let mut resolved: #struct_name #type_generics = ::std::default::Default::default();
                    let maybe_annotations = ::compris::annotate::Annotated::maybe_annotations(&self);

                    let mut map = match self {
                        ::compris::normal::Variant::Map(map) => map,
                        _ => {
                            #single_field
                        }
                    };

                    #(#segments)*

                    ::compris::resolve::ResolveResult::Ok(
                        ::std::option::Option::Some(resolved)
                    )
                }
            }
        }
    }
}
