use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::new();

        let annotated_parameter = self.annotated_parameter();
        let (impl_generics, type_generics, where_clause) = self.generics(&annotated_parameter);

        let enum_name = &self.enum_name;
        let quoted_enum_name = enum_name.to_string().to_token_stream();
        let human_readable_key_list = &self.human_readable_key_list;

        let handle_single_variant = match &self.single_variant {
            Some(single_variant) => self.generate_handle_single_variant(single_variant),
            None => TokenStream::new(),
        };

        for select_variant in &self.select_variants {
            segments.push(self.generate_handle_variant(select_variant));
        }

        quote! {
            #[automatically_derived]
            impl
                #impl_generics
                Resolve<#enum_name #type_generics, #annotated_parameter>
                for ::compris::normal::Variant<#annotated_parameter>
                #where_clause
            {
                fn resolve_with_errors<ErrorRecipientT>(&self, errors: &mut ErrorRecipientT) ->
                    ::compris::resolve::ResolveResult<#enum_name #type_generics, #annotated_parameter>
                    where ErrorRecipientT:
                        ::kutil_std::error::ErrorRecipient<::compris::resolve::ResolveError<#annotated_parameter>>
                {
                    #handle_single_variant

                    ::compris::resolve::ResolveResult::Ok(
                        match self.to_key_value_pair() {
                            Some((key, value)) => match key {
                                Self::Text(text) => match text.as_str() {
                                    #(#segments)*

                                    key => {
                                        errors.give(
                                            ::compris::annotate::Annotated::with_annotations_from(
                                                ::compris::normal::MalformedError::new(
                                                    #quoted_enum_name.into(),
                                                    format!("key is not {}: {}", #human_readable_key_list, key),
                                                ),
                                                self
                                            ),
                                        )?;
                                        None
                                    }
                                }

                                _ => {
                                    errors.give(
                                        ::compris::annotate::Annotated::with_annotations_from(
                                            ::compris::normal::IncompatibleVariantTypeError::new(
                                                self,
                                                &["text"]
                                            ),
                                            self,
                                        ),
                                    )?;
                                    None
                                }
                            }

                            None => {
                                errors.give(
                                    ::compris::annotate::Annotated::with_annotations_from(
                                        ::compris::normal::MalformedError::new(
                                            "map".into(),
                                            "is not a single-key map".into(),
                                        ),
                                        self
                                    ),
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
