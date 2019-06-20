extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(KVPredicatesSimple)]
pub fn kv_predicates_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kv_predicates(&ast)
}

fn impl_kv_predicates(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl KVPredicatesSimple for #name {
            fn test() -> String {
                format!("Hello, Macro! My name is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}
