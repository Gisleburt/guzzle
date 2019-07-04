extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Type};

#[proc_macro_derive(YummyMetadata, attributes(yummy))]
pub fn yummy_metadata_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_yummy_metadata(&ast)
}

fn field_to_type_and_ident(field: &Field) -> Option<(&Type, &Ident)> {
    if let Some(ident) = field.ident.as_ref() {
        Some((&field.ty, ident))
    } else {
        None
    }
}

fn impl_yummy_metadata(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => impl_yummy_metadata_named_fields(ast, fields),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn impl_yummy_metadata_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (matchers, keys): (Vec<String>, Vec<&Ident>) = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref())
        .filter_map(|ident| ident)
        .map(|ident| (ident.to_string(), ident))
        .unzip();

    let gen = quote! {
        impl #impl_generics YummyMetadata for #name #ty_generics #where_clause {
            fn eat_yummy_metadata<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
            where T: AsRef<str>
            {
                match key.as_ref() {
                    #( #matchers => self.#keys = value, )*
                    _ => return Some((key, value)),
                };
                None
            }
        }
    };
    gen.into()
}
