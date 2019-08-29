extern crate proc_macro;

mod attr;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, LitStr,
    Meta, MetaNameValue,
};

/// This structure models the guzzle attribute.
///
/// ```
/// #[derive(Guzzle)]
/// struct Guzzle {
///     /// This field is not annotated, therefore its field is `basic` and its keys contain
///     /// one string which is the same as the name `basic`.
///     basic: String,
///     /// This field may be filled from multiple keys
///     #[guzzle(keys = ["one", "two"])]
///     listed_keys: String,
///     /// This field is not a string, you must provider a parser that will transform it into
///     /// the correct type
///     #[guzzle(parser = "my_parser")]
///     other_types: u64,
///     /// This field isn't a string and has multiple keys
///     #[guzzle(parser = "my_parser", keys = ["three", "four"])]
///     other_types_with_listed_keys: u64,
/// }
/// ```
struct GuzzleAttribute<'a> {
    field: &'a Ident,
    keys: Vec<LitStr>,
}

impl<'a> GuzzleAttribute<'a> {
    fn get_arm_parts(&self) -> Vec<(&Ident, &LitStr)> {
        self.keys
            .iter()
            .map(|matcher| (self.field, matcher))
            .collect()
    }
}

impl<'a> From<&'a Field> for GuzzleAttribute<'a> {
    fn from(field: &'a Field) -> Self {
        let meta = get_guzzle_meta(field.attrs.as_ref());
        let (keys, field) = get_key_and_field(field.ident.as_ref().unwrap(), &meta);
        GuzzleAttribute { field, keys }
    }
}

#[proc_macro_derive(Guzzle, attributes(guzzle))]
pub fn guzzle_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input);

    // Build the trait implementation
    impl_guzzle(&ast)
}

fn impl_guzzle(ast: &DeriveInput) -> TokenStream {
    match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fields) => impl_guzzle_named_fields(ast, fields),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

fn ident_to_str(ident: &Ident) -> LitStr {
    LitStr::new(ident.to_string().as_ref(), ident.span())
}

fn get_guzzle_meta(attrs: &[Attribute]) -> Vec<MetaNameValue> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("guzzle"))
        .filter_map(|attr| {
            attr.parse_meta()
                .map_err(|e| {
                    println!("nope");
                    e
                })
                .ok()
        })
        .map(|v| {
            println!("yes");
            v
        })
        .filter_map(|meta| {
            if let Meta::NameValue(name_value) = meta {
                println!("hello");
                Some(name_value)
            } else {
                None
            }
        })
        .collect()
}

fn names_from_meta(meta: &[MetaNameValue]) -> Vec<LitStr> {
    meta.iter().fold(Vec::new(), |mut acc, l| {
        println!("meta - {}", l.ident);
        acc
    })
}

fn get_key_and_field<'a, 'b>(
    ident: &'a Ident,
    meta: &'b [MetaNameValue],
) -> (Vec<LitStr>, &'a Ident) {
    let mut name = names_from_meta(meta);
    if name.is_empty() {
        name.push(ident_to_str(ident))
    }
    (name, ident)
}

fn fields_to_attributes(fields: &FieldsNamed) -> Vec<GuzzleAttribute> {
    fields.named.iter().map(|field| field.into()).collect()
}

fn impl_guzzle_named_fields(ast: &DeriveInput, fields: &FieldsNamed) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let attributes = fields_to_attributes(fields);

    let mut keys_and_matchers = Vec::new();

    for attr in &attributes {
        keys_and_matchers.append(&mut attr.get_arm_parts());
    }

    let (keys, matchers): (Vec<_>, Vec<_>) = keys_and_matchers.into_iter().unzip();

    let gen = quote! {
        impl #impl_generics Guzzle for #name #ty_generics #where_clause {
            fn guzzle<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
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
