use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate `impl Resolve`.
    pub fn generate_impl_resolve(&self) -> TokenStream {
        let mut segments = Vec::<TokenStream>::default();

        let annotated_parameter = self.annotated_parameter();
        let (impl_generics, type_generics, where_clause) = self.generics(&annotated_parameter);

        let enum_name = &self.enum_name;
        let quoted_enum_name = enum_name.to_string().to_token_stream();
        let human_readable_key_list = &self.human_readable_key_list;

        let handle_single_variant = match &self.single_variant {
            Some(single_variant) => self.generate_handle_single_variant(single_variant),
            None => Default::default(),
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
                fn resolve_with_errors<ErrorRecipientT>(self, errors: &mut ErrorRecipientT) ->
                    ::compris::resolve::ResolveResult<#enum_name #type_generics, #annotated_parameter>
                    where ErrorRecipientT:
                        ::kutil::std::error::ErrorRecipient<::compris::resolve::ResolveError<#annotated_parameter>>
                {
                    let maybe_annotations = ::compris::annotate::Annotated::maybe_annotations(&self);
                    let type_name = self.type_name();

                    #handle_single_variant

                    ::compris::resolve::ResolveResult::Ok(
                        match self.into_key_value_pair() {
                            ::std::option::Option::Some((key, value)) => match key {
                                Self::Text(text) => match text.as_str() {
                                    #(#segments)*

                                    key => {
                                        ::kutil::std::error::ErrorRecipient::give_error(
                                            errors,
                                            ::compris::annotate::Annotated::with_annotations_from(
                                                ::compris::normal::MalformedError::new(
                                                    #quoted_enum_name.into(),
                                                    format!("key is not {}: {}", #human_readable_key_list, key),
                                                ).into(),
                                                &maybe_annotations
                                            ),
                                        )?;
                                        ::std::option::Option::None
                                    }
                                }

                                _ => {
                                    ::kutil::std::error::ErrorRecipient::give_error(
                                        errors,
                                        ::compris::annotate::Annotated::with_annotations_from(
                                            ::compris::normal::IncompatibleVariantTypeError::new(
                                                type_name.into(),
                                                vec!["text".into()]
                                            ).into(),
                                            &maybe_annotations,
                                        ),
                                    )?;
                                    ::std::option::Option::None
                                }
                            }

                            ::std::option::Option::None => {
                                ::kutil::std::error::ErrorRecipient::give_error(
                                    errors,
                                    ::compris::annotate::Annotated::with_annotations_from(
                                        ::compris::normal::MalformedError::new(
                                            "map".into(),
                                            "is not a single-key map".into(),
                                        ).into(),
                                        &maybe_annotations
                                    ),
                                )?;
                                ::std::option::Option::None
                            }
                        }
                    )
                }
            }
        }
    }
}
