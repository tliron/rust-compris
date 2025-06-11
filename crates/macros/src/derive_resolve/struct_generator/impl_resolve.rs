use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::new();

        // Cite self as empty-string field
        if let Some(citations_field_name) = &self.citations_field {
            segments.push(quote! {
                resolved.#citations_field_name.insert(
                    "".into(),
                    ::compris::cite::Citation::new_for(self, context, ancestor),
                );
            });
        }

        let struct_name = &self.struct_name;
        let (context, error, impl_generics) = self.generate_impl_resolve_generics();

        if let Some(single_field) = &self.single_field {
            segments.push(self.generate_handle_single_field(single_field, &context, &error));
        }

        for (resolve_field, key) in &self.resolve_fields {
            segments.push(self.generate_handle_field(resolve_field, key));
        }

        if !self.struct_attribute.ignore_other_keys {
            segments.push(self.generate_handle_other_keys());
        }

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                Resolve<#struct_name, #context, #error>
                for ::compris::normal::Value
            {
                fn
                    resolve_for
                    <'own, ErrorRecipientT>
                    (
                        &'own self,
                        context: ::std::option::Option<&'own #context>,
                        mut ancestor: ::std::option::Option<&'own ::compris::normal::Value>,
                        errors: &mut ErrorRecipientT
                    )
                    -> ::compris::resolve::ResolveResult<#struct_name, #error>
                    where ErrorRecipientT: ::kutil_std::error::ErrorRecipient<#error>
                {
                    if ancestor.is_none() {
                        ancestor = Some(self)
                    }

                    let mut resolved: #struct_name = ::std::default::Default::default();

                    #(#segments)*

                    ::compris::resolve::ResolveResult::Ok(
                        ::std::option::Option::Some(resolved)
                    )
                }
            }
        }
    }
}
