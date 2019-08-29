extern crate proc_macro;

mod attr;

use crate::proc_macro::TokenStream;
use attr::GuzzleAttributes;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, LitStr};

/// This structure models the guzzle attribute. This is a work in progress and not only won't
/// function as is, but may change dramatically.
///
/// ```ignore
/// #[derive(Guzzle)]
/// struct Guzzle {
///     /// This field is not annotated, therefore it will not be parsed by guzzle
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
struct FieldAttribute<'a> {
    field: &'a Ident,
    attributes: GuzzleAttributes,
}

impl<'a> FieldAttribute<'a> {
    fn get_arm_parts(&self) -> Vec<(&Ident, &LitStr)> {
        self.attributes
            .keys
            .iter()
            .map(|matcher| (self.field, matcher))
            .collect()
    }
}

impl<'a> From<&'a Field> for FieldAttribute<'a> {
    fn from(field: &'a Field) -> Self {
        let mut attributes = GuzzleAttributes::new();
        let all_attrs = &field.attrs;
        for attr in all_attrs {
            let path = &attr.path;
            if let "guzzle" = quote!(#path).to_string().as_ref() {
                let tokens = attr.tts.clone();
                let is_empty = tokens.is_empty();
                attributes = syn::parse2(tokens).unwrap_or_else(|err| {
                    let tokens_str = if is_empty {
                        String::new()
                    } else {
                        format!("problematic tokens: {}", &attr.tts)
                    };
                    panic!("{}, {}", err.to_string(), tokens_str)
                });
            }
        }
        let field = field.ident.as_ref().unwrap();
        FieldAttribute { field, attributes }
    }
}

#[proc_macro_derive(Guzzle, attributes(guzzle, noguzzle))]
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
