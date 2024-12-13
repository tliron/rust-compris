mod derive_resolve;
mod utils;

use derive_resolve::*;

// See: https://petanode.com/posts/rust-proc-macro/

/// Procedural macro for `#[derive(Resolve)]`.
#[proc_macro_derive(Resolve, attributes(resolve))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse_macro_input!(input);

    match input.data {
        syn::Data::Struct(_) => StructGenerator::generate(&mut input),

        syn::Data::Enum(_) => EnumGenerator::generate(&mut input),

        _ => Err(syn::Error::new(input.ident.span(), "`Resolve`: not a struct or an enum")),
    }
    .unwrap_or_else(|e| e.to_compile_error())
    .into()
}
