extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, Ident, Meta, NestedMeta,
};

#[proc_macro_derive(YummyMetadata, attributes(yummy))]
pub fn yummy_metadata_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_yummy_metadata(&ast)
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

fn name_from_attrs(attrs: &[Attribute]) -> Option<String> {
    // 1. Get the first result
    // 2. Fail if another result is found.

    attrs
        .iter()
        .map(|attr| (attr.parse_meta(), attr))
        .filter_map(|(r, a)| r.ok().map(|m| (m, a)))
        .filter(|(m, a)| m.name() == Ident::new("yummy", a.span()))
        .filter_map(|(m, _a)| if let Meta::List(l) = m { Some(l) } else { None })
        .filter_map(|l| {
            if l.nested.len() > 1 {
                panic!("you must provide one, and only one name when using the yummy attribute");
            }
            if let Some(n) = l.nested.first() {
                if let NestedMeta::Meta(m) = n.value() {
                    if let Meta::Word(w) = m {
                        return Some(w.to_string());
                    }
                }
            }
            None
        })
        .next()
}

fn get_key_and_field<'a, 'b>(ident: &'a Ident, attrs: &'b [Attribute]) -> (String, &'a Ident) {
    let name = name_from_attrs(attrs).unwrap_or_else(|| ident.to_string());
    (name, ident)
}

fn fields_to_names_and_idents(fields: &FieldsNamed) -> (Vec<String>, Vec<&Ident>) {
    fields
        .named
        .iter()
        .map(|field| (field.ident.as_ref(), field.attrs.as_ref()))
        .map(|(ident, attr)| get_key_and_field(ident.unwrap(), &attr))
        .unzip()
}

fn impl_yummy_metadata_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let (matchers, keys): (Vec<String>, Vec<&Ident>) = fields_to_names_and_idents(fields);

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
