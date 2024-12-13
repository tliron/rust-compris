use super::{generator::*, impl_resolve_generics::*};

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::new();

        let enum_name = &self.enum_name;

        let (context, error, impl_generics) = self.generate_impl_resolve_generics();

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

                    ::compris::resolve::ResolveResult::Ok(match self {
                        Self::Map(map) => match map.value.len() {
                            1 => {
                                let (key, value) = map.value.iter().next().unwrap();
                                match key {
                                    Self::Text(text) => match text.value.as_str() {
                                        #(#segments)*

                                        "content" => match ::compris::resolve::Resolve::resolve_for(value, context, ancestor, errors)? {
                                            Some(x) => ::std::option::Option::Some(File::Content(x)),
                                            None => None,
                                        },

                                        key => {
                                            errors.give(
                                                ::compris::normal::MalformedError::new(
                                                    "BindPem",
                                                    &format!("key is not \"content\" or \"path\": {}", key),
                                                )
                                                .with_citation_for(self, context, ancestor),
                                            )?;
                                            None
                                        }
                                    },

                                    _ => {
                                        errors.give(
                                            ::compris::normal::IncompatibleValueTypeError::new(self, &["text"])
                                                .with_citation_for(self, context, ancestor),
                                        )?;
                                        None
                                    }
                                }
                            }

                            length => {
                                errors.give(
                                    ::compris::normal::MalformedError::new("map", &format!("length is not 1: {}", length))
                                        .with_citation_for(self, context, ancestor),
                                )?;
                                None
                            }
                        }

                        _ => {
                            errors.give(
                                ::compris::normal::IncompatibleValueTypeError::new(self, &["map"])
                                    .with_citation_for(self, context, ancestor),
                            )?;
                            None
                        }
                    })
                }
            }
        }
    }
}
