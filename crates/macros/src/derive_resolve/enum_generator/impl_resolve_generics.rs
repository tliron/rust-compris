use super::generator::*;

use {proc_macro2::*, quote::*};

impl EnumGenerator {
    /// Generate generics for `impl Resolve`.
    pub fn generate_impl_resolve_generics(&self) -> (TokenStream, TokenStream, TokenStream) {
        let (is_context_generic, context, generic_context) = match &self.enum_attribute.context {
            Some(context) => (false, quote! { #context }, TokenStream::new()),
            None => (true, quote! { ContextT }, quote! { ContextT: ::compris::resolve::ResolveContext }),
        };

        let (is_error_generic, error, generic_error) = match &self.enum_attribute.error {
            Some(error) => (false, quote! { #error }, TokenStream::new()),
            None => (true, quote! { ErrorT }, quote! { ErrorT: ::compris::resolve::ResolveError }),
        };

        if is_context_generic && is_error_generic {
            (context, error, quote! { < #generic_context, #generic_error > })
        } else if is_context_generic {
            (context, error, quote! { < #generic_context > })
        } else if is_error_generic {
            (context, error, quote! { < #generic_error > })
        } else {
            // No generics
            (context, error, TokenStream::new())
        }
    }
}
