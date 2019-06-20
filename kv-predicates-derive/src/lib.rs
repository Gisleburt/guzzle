extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields, Ident};

#[proc_macro_derive(KVPredicatesSimple)]
pub fn kv_predicates_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kv_predicates(&ast)
}

fn impl_kv_predicates(ast: &syn::DeriveInput) -> TokenStream {
    if let Data::Struct(s) = &ast.data {
        if let Fields::Named(named) = &s.fields {
            let field_names: Vec<String> = named
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .map(|i| format!("{}", i))
                .collect();
            let name = &ast.ident;
            let gen = quote! {
                impl KVPredicatesSimple for #name {
                    fn test() -> String {
                        concat!(
                            "This is a ",
                            stringify!(#name),
                            ", it contains",
                            #(" ", #field_names),*
                        ).to_string()
                    }
                }
            };
            gen.into()
        } else {
            panic!("kv_predicates currently only work for structs")
        }
    } else {
        panic!("kv_predicates currently only work for structs")
    }
}
