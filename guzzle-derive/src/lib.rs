extern crate proc_macro;

use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};
use crate::proc_macro::TokenStream;
use crate::attr::FieldAttribute;
use std::convert::TryInto;

mod attr;

#[proc_macro_derive(Guzzle, attributes(guzzle, no_guzzle, deep_guzzle))]
pub fn guzzle_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as an abstract syntax tree
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

fn fields_to_attributes(fields: &FieldsNamed) -> Result<Vec<FieldAttribute>, Vec<syn::Error>> {
    let mut oks = vec![];
    let mut errs = vec![];
    fields.named.iter().for_each(|field| {
        match field.try_into() {
            Ok(field_attribute) => oks.push(field_attribute),
            Err(error) => errs.push(error),
        };
    });

    if !errs.is_empty() {
        Err(errs)
    } else {
        Ok(oks)
    }
}

fn handle_errors(errors: Vec<syn::Error>) -> TokenStream {
    let mut output = TokenStream::new();
    errors.iter().for_each(|error| {
        let ts = quote_spanned! {
            error.span() => compile_error!("hello")
        };
        output.extend(TokenStream::from(ts));
    });
    output
}

fn impl_guzzle_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let results = fields_to_attributes(fields);
    if results.is_err() {
        return handle_errors(results.err().unwrap());
    }
    let attributes = results.unwrap();

    let mut deep_guzzles = vec![];
    let mut keys = vec![];
    let mut matchers = vec![];
    let mut parsers = vec![];

    for field_attribute in &attributes {
        // In the future we might have types of attributes so this might need opening up but it'll
        // do for now.
        if let Some(expr) = field_attribute.get_recursion() {
            deep_guzzles.push(expr);
        } else {
            for (key, matcher, parser) in field_attribute.get_arm_parts() {
                keys.push(key);
                matchers.push(matcher);
                parsers.push(parser);
            }
        }
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
