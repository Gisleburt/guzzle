extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Field, Fields, Ident, Type};

#[proc_macro_derive(KVPredicates)]
pub fn kv_predicates_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_kv_predicates(&ast)
}

fn field_to_type_and_ident(field: &Field) -> Option<(&Type, &Ident)> {
    if let Some(ident) = field.ident.as_ref() {
        Some((&field.ty, ident))
    } else {
        None
    }
}

fn impl_kv_predicates(ast: &syn::DeriveInput) -> TokenStream {
    if let Data::Struct(s) = &ast.data {
        if let Fields::Named(fields) = &s.fields {
            let (_types, idents): (Vec<&Type>, Vec<&Ident>) = fields
                .named
                .iter()
                .filter_map(field_to_type_and_ident)
                .unzip();
            let (keys, idents): (Vec<String>, Vec<&Ident>) =
                idents.into_iter().map(|i| (format!("{}", i), i)).unzip();
            println!("{:?}", keys);
            let name = &ast.ident;
            let gen = quote! {
                impl KVPredicates for #name {
                    fn filter_map_predicate<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
                    where T: AsRef<str>
                    {
                        match key.as_ref() {
                            #( #keys => self.#idents = value, )*
                            _ => return Some((key, value)),
                        };
                        None
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
