use super::generator::*;

use {
    proc_macro2::*,
    quote::*,
    syn::{punctuated::*, token::*, *},
};

impl EnumGenerator {
    /// Generic parameter for annotations.
    pub fn annotated_parameter(&self) -> TokenStream {
        self.enum_attribute
            .annotated_parameter
            .as_ref()
            .map(|parameter| parameter.to_token_stream())
            .unwrap_or(quote! {_AnnotatedT})
    }

    /// impl_generics, type_generics, where_clause.
    pub fn generics(&self, annotated_parameter: &TokenStream) -> (TokenStream, TokenStream, TokenStream) {
        let mut struct_generics = self.enum_generics.clone();

        if self.enum_attribute.annotated_parameter.is_none() {
            struct_generics.params.push(parse_quote! {#annotated_parameter});
        }

        let (impl_generics, _, _) = struct_generics.split_for_impl();
        let (_, type_generics, where_clause) = self.enum_generics.split_for_impl();

        let mut where_clause = where_clause
            .cloned()
            .unwrap_or(WhereClause { where_token: Where::default(), predicates: Punctuated::default() });

        where_clause.predicates.push(parse_quote! {
            #annotated_parameter:
                ::compris::annotate::Annotated
                + ::std::clone::Clone
                + ::std::fmt::Debug
                + ::std::default::Default
        });

        (impl_generics.into_token_stream(), type_generics.into_token_stream(), where_clause.into_token_stream())
    }
}
