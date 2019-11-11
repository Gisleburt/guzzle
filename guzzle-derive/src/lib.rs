extern crate proc_macro;

mod attr;

use crate::proc_macro::TokenStream;
use crate::attr::FieldAttribute;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Guzzle, attributes(guzzle, no_guzzle, deep_guzzle))]
pub fn guzzle_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_guzzle(ast)
}

fn impl_guzzle(ast: DeriveInput) -> TokenStream {
    match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => impl_guzzle_named_fields(&ast, fields),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn fields_to_attributes(fields: &FieldsNamed) -> Vec<FieldAttribute> {
    fields.named.iter().map(|field| field.into()).collect()
}

fn impl_guzzle_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let attributes = fields_to_attributes(fields);

    let mut keys_and_matchers = Vec::new();
    let mut deep_guzzles = Vec::new();

    for attr in &attributes {
        keys_and_matchers.append(&mut attr.get_arm_parts());
        if let Some(expr) = attr.get_recursion() {
            deep_guzzles.push(expr);
        }
    }

    let mut keys = vec![];
    let mut matchers = vec![];
    let mut parsers = vec![];
    for (key, matcher, parser) in keys_and_matchers {
        keys.push(key);
        matchers.push(matcher);
        parsers.push(parser);
    }

    let gen = quote! {
        impl #impl_generics Guzzle for #name #ty_generics #where_clause {
            fn guzzle<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
            where T: AsRef<str>
            {
                #(
                    let result = self.#deep_guzzles.guzzle((key, value));
                    if result.is_none() { return None };
                    let (key, value) = result.unwrap();
                )*
                match key.as_ref() {
                    #( #matchers => self.#keys = #parsers(value), )*
                    _ => return Some((key, value)),
                };
                None
            }
        }
    };
    gen.into()
}
