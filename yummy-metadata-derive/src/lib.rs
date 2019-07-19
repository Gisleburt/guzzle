extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Lit,
    LitStr, Meta, MetaList, NestedMeta,
};

struct YummyAttribute<'a> {
    field: &'a Ident,
    keys: Vec<LitStr>,
}

impl<'a> YummyAttribute<'a> {
    fn get_arm_parts(&self) -> Vec<(&Ident, &LitStr)> {
        self.keys
            .iter()
            .map(|matcher| (self.field, matcher))
            .collect()
    }
}

impl<'a> From<&'a Field> for YummyAttribute<'a> {
    fn from(field: &'a Field) -> Self {
        let meta = get_yummy_meta(field.attrs.as_ref());
        let (keys, field) = get_key_and_field(field.ident.as_ref().unwrap(), &meta);
        YummyAttribute { field, keys }
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

fn ident_to_str(ident: &Ident) -> LitStr {
    LitStr::new(ident.to_string().as_ref(), ident.span())
}

fn get_yummy_meta(attrs: &[Attribute]) -> Vec<MetaList> {
    attrs
        .iter()
        .map(|attr| (attr.parse_meta(), attr))
        .filter_map(|(r, a)| r.ok().map(|m| (m, a)))
        .filter(|(m, a)| m.name() == Ident::new("yummy", a.span()))
        .filter_map(|(m, _a)| if let Meta::List(l) = m { Some(l) } else { None })
        .collect()
}

fn names_from_meta(meta: &[MetaList]) -> Vec<LitStr> {
    meta.iter().fold(Vec::new(), |mut acc, l| {
        l.nested.iter().for_each(|n| match n {
            NestedMeta::Literal(Lit::Str(x)) => acc.push(x.to_owned()),
            _ => panic!("Expected a string literal"),
        });
        acc
    })
}

fn get_key_and_field<'a, 'b>(ident: &'a Ident, meta: &'b [MetaList]) -> (Vec<LitStr>, &'a Ident) {
    let mut name = names_from_meta(meta);
    if name.is_empty() {
        name.push(ident_to_str(ident))
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
