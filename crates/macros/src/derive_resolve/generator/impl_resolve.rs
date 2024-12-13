use super::generator::*;

use {proc_macro2::*, quote::*};

impl Generator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::new();

        for (resolve_field, key) in &self.resolve_fields {
            segments.push(self.generate_handle_field(resolve_field, key));
        }

        if !self.struct_attribute.ignore_other_keys {
            segments.push(self.generate_handle_other_keys());
        }

        let struct_name = &self.struct_name;
        let (context, error, impl_generics) = self.generate_impl_resolve_generics();

        quote! {
            impl
                #impl_generics
                Resolve<#struct_name, #context, #error>
                for ::compris::normal::Value
            {
                fn
                    resolve_for
                    <'a, ErrorRecipientT>
                    (
                        &'a self,
                        context: ::std::option::Option<&'a #context>,
                        mut ancestor: ::std::option::Option<&'a ::compris::normal::Value>,
                        errors: &mut ErrorRecipientT
                    )
                    -> ::compris::resolve::ResolveResult<#struct_name, #error>
                    where ErrorRecipientT: ::compris::resolve::ErrorRecipient<#error>
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
