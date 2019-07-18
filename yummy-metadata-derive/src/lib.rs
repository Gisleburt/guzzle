extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta,
    NestedMeta,
};

struct YummyAttribute<'a> {
    key: &'a Ident,
    matchers: Vec<String>,
}

impl<'a> YummyAttribute<'a> {
    fn get_arm_parts(&self) -> Vec<(&Ident, &String)> {
        self.matchers
            .iter()
            .map(|matcher| (self.key, matcher))
            .collect()
    }
}

impl<'a> From<&'a Field> for YummyAttribute<'a> {
    fn from(field: &'a Field) -> Self {
        let (matchers, key) =
            get_key_and_field(field.ident.as_ref().unwrap(), field.attrs.as_ref());
        YummyAttribute { key, matchers }
    }
}

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

fn names_from_attrs(attrs: &[Attribute]) -> Vec<String> {
    // 1. Get the first result
    // 2. Fail if another result is found.

    attrs
        .iter()
        .map(|attr| (attr.parse_meta(), attr))
        .filter_map(|(r, a)| r.ok().map(|m| (m, a)))
        .filter(|(m, a)| m.name() == Ident::new("yummy", a.span()))
        .filter_map(|(m, _a)| if let Meta::List(l) = m { Some(l) } else { None })
        .fold(Vec::new(), |mut acc, l| {
            l.nested.iter().for_each(|n| {
                if let NestedMeta::Meta(m) = n {
                    if let Meta::Word(w) = m {
                        acc.push(w.to_string());
                    }
                }
            });
            acc
        })
}

fn get_key_and_field<'a, 'b>(ident: &'a Ident, attrs: &'b [Attribute]) -> (Vec<String>, &'a Ident) {
    let mut name = names_from_attrs(attrs);
    if name.is_empty() {
        name.push(ident.to_string())
    }
    (name, ident)
}

fn fields_to_attributes(fields: &FieldsNamed) -> Vec<YummyAttribute> {
    fields.named.iter().map(|field| field.into()).collect()
}

fn impl_yummy_metadata_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let attributes = fields_to_attributes(fields);

    let mut keys_and_matchers = Vec::new();

    for attr in &attributes {
        keys_and_matchers.append(&mut attr.get_arm_parts());
    }

    let (keys, matchers): (Vec<_>, Vec<_>) = keys_and_matchers.into_iter().unzip();

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
