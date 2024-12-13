mod field_attribute;
mod generate_field_segment;
mod generate_impl_resolve;
mod struct_attribute;

// See: https://petanode.com/posts/rust-proc-macro/

use generate_impl_resolve::*;

#[proc_macro_derive(Resolve, attributes(resolve))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: syn::DeriveInput = syn::parse_macro_input!(input);
    generate_impl_resolve(&mut input).unwrap_or_else(|e| e.to_compile_error()).into()
}
