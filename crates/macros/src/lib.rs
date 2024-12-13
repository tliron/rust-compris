mod derive_resolve;
mod utils;

// See: https://petanode.com/posts/rust-proc-macro/

/// Procedural macro for `#[derive(Resolve)]`.
#[proc_macro_derive(Resolve, attributes(resolve))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut item: syn::ItemStruct = syn::parse_macro_input!(input);
    derive_resolve::Generator::generate(&mut item).unwrap_or_else(|e| e.to_compile_error()).into()
}
