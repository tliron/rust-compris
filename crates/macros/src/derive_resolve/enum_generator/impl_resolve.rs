use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::new();

        let enum_name = &self.enum_name;
        let quoted_enum_name = enum_name.to_string().to_token_stream();
        let human_readable_key_list = &self.human_readable_key_list;
        let (context, error, impl_generics) = self.generate_impl_resolve_generics();

        for select_variant in &self.select_variants {
            segments.push(self.generate_handle_variant(select_variant));
        }

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                Resolve<#enum_name, #context, #error>
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
                    -> ::compris::resolve::ResolveResult<#enum_name, #error>
                    where ErrorRecipientT: ::kutil_std::error::ErrorRecipient<#error>
                {
                    if ancestor.is_none() {
                        ancestor = Some(self)
                    }

                    ::compris::resolve::ResolveResult::Ok(
                        match self.to_key_value_pair() {
                            Some((key, value)) => match key {
                                Self::Text(text) => match text.value.as_str() {
                                    #(#segments)*

                                    key => {
                                        errors.give(
                                            ::compris::normal::MalformedError::new(
                                                #quoted_enum_name,
                                                &format!("key is not {}: {}", #human_readable_key_list, key),
                                            )
                                            .with_citation_for(self, context, ancestor),
                                        )?;
                                        None
                                    }
                                }

                                _ => {
                                    errors.give(
                                        ::compris::normal::IncompatibleValueTypeError::new(
                                            self,
                                            &["text"]
                                        )
                                        .with_citation_for(self, context, ancestor),
                                    )?;
                                    None
                                }
                            }

                            None => {
                                errors.give(
                                    ::compris::normal::MalformedError::new(
                                        "map",
                                        "is not a single-key map",
                                    )
                                    .with_citation_for(self, context, ancestor),
                                )?;
                                None
                            }
                        }
                    )
                }
            }
        }
    }
}
