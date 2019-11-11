use std::default::Default;
use std::fmt::{Debug, Error as FormatError, Formatter};
use std::ops::Deref;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseBuffer},
    punctuated::Punctuated,
    Expr, Field, Ident, LitStr, Token,
};
use quote::quote;

/// This structure models the guzzle attribute. This is a work in progress and not only won't
/// function as is, but may change dramatically.
///
/// ```ignore
/// #[derive(Guzzle)]
/// struct GuzzleExample {
///     /// This field is annotated with noguzzle, therefore it will not be parsed by guzzle
///     #[noguzzle]
///     ignored: String,
///
///     /// This field is not annotated, so if a key matches the field name, it will set the value
///     basic: String,
///
///     /// This field is annotated, but with no keys, so it uses the field name as a key
///     #[guzzle]
///     basic_too: String,
///
///     /// This field may be filled from multiple keys
///     #[guzzle(keys = ["one", "two"])]
///     listed_keys: String,
///
///     /// This field is not a string, you must provider a parser that will transform it into
///     /// the correct type
///     #[guzzle(parser = "my_parser")]
///     other_types: u64,
///
///     /// This field isn't a string and has multiple keys
///     #[guzzle(parser = "my_parser", keys = ["three", "four"])]
///     other_types_with_listed_keys: u64,
///
///     /// Guzzle will wire up this field so that data being guzzled by the `GuzzleExample`
///     /// will first be sent to the `TypeThatAlsoImplementsGuzzle`. If the
///     /// `TypeThatAlsoImplementsGuzzle` consumes the value, `GuzzleExample` will not.
///     #[deep_guzzle]
///     recurse_guzzle_to_populate_this_field: TypeThatAlsoImplementsGuzzle,
/// }
/// ```
pub struct FieldAttribute<'a> {
    field: &'a Ident,
    attributes: Option<GuzzleAttribute>,
}

impl<'a> FieldAttribute<'a> {
    pub fn get_arm_parts(&self) -> Vec<(&Ident, &LitStr, &Option<Expr>)> {
        self.attributes
            .as_ref()
            .map(|attr| {
                // We only want to get arm parts from keyed attributes
                if let GuzzleAttribute::KeyedAttribute(keyed_attr) = attr {
                    keyed_attr.keys
                        .iter()
                        .map(|matcher| (self.field, matcher, &keyed_attr.parser))
                        .collect()
                } else {
                    vec![]
                }
            })
            .unwrap_or_default()
    }

    pub fn get_recursion(&self) -> Option<&Ident> {
        self.attributes
            .as_ref()
            .map(|attr| {
                if let GuzzleAttribute::RecurseAttribute(expr) = attr {
                    Some(expr)
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
}

impl<'a> From<&'a Field> for FieldAttribute<'a> {
    fn from(field: &'a Field) -> Self {
        // Default value for keys is just the name of the field
        let name_ident = field.ident.clone().unwrap();

        // Unless otherwise turned off we'll default to a keyed attribute with the same name as the
        // field (see below)
        let mut attributes = Some(GuzzleAttribute::default());

        let all_attrs = &field.attrs;
        for attr in all_attrs {
            let path = &attr.path;
            match quote!(#path).to_string().as_ref() {
                "guzzle" => {
                    let tokens = attr.tts.clone();
                    let is_empty = tokens.is_empty();
                    attributes = Some(
                        GuzzleAttribute::KeyedAttribute(
                            syn::parse2(tokens).unwrap_or_else(|err| {
                                let tokens_str = if is_empty {
                                    String::new()
                                } else {
                                    format!("problematic tokens: {}", &attr.tts)
                                };
                                panic!("{}, {}", err.to_string(), tokens_str)
                            })
                        )
                    );
                }
                "deep_guzzle" => {
                    attributes = Some(GuzzleAttribute::RecurseAttribute(name_ident.clone()));
                    break;
                }
                "noguzzle" => {
                    attributes = None;
                    break;
                }
                _ => {}
            }
        }
        // If attributes are Some, make sure at least one key is available by using the field name
        attributes = attributes.map(|attr| {
            if let GuzzleAttribute::KeyedAttribute(mut keyed_attr) = attr {
                keyed_attr.set_default_key_if_none(name_ident);
                GuzzleAttribute::KeyedAttribute(keyed_attr)
            } else {
                attr
            }
        });

        let field = field.ident.as_ref().unwrap();
        FieldAttribute { field, attributes }
    }
}

/// If a field has a `guzzle` attribute it must be either a keyed attribute, or a recursive
/// attribute.
/// ```ignore
/// #[derive(Guzzle)]
/// struct GuzzleExample {
///     /// This is a KeyedAttribute
///     #[guzzle(keys = ["one", "two"])]
///     listed_keys: String,
///
///     /// This is also a KeyedAttribute, its key will default to the field name
///     basic: String,
///
///     /// This is a RecurseAttribute
///     #[deep_guzzle]
///     recurse_guzzle_to_populate_this_field: TypeThatAlsoImplementsGuzzle,
/// }
/// ```
pub enum GuzzleAttribute {
    KeyedAttribute(GuzzleKeyedAttribute),
    RecurseAttribute(Ident),
}

impl Default for GuzzleAttribute {
    fn default() -> Self {
        GuzzleAttribute::KeyedAttribute(
            GuzzleKeyedAttribute::default()
        )
    }
}

#[derive(Default)]
pub struct GuzzleKeyedAttribute {
    pub keys: Keys,
    pub parser: Option<Expr>,
}

impl GuzzleKeyedAttribute {
    pub fn set_default_key_if_none(&mut self, ident: Ident) {
        if self.keys.is_empty() {
            self.keys = Keys(vec![LitStr::new(ident.to_string().as_str(), ident.span())])
        }
    }
}

impl Parse for GuzzleKeyedAttribute {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let mut guzzle_attributes = GuzzleKeyedAttribute::default();
        // the guzzle attribute may have brackets containing more details or it may not
        // eg `#[guzzle()]` or  `#[guzzle]
        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            let punctuated_attrs: Punctuated<RawGuzzleKeyedAttribute, Token![,]> =
                content.parse_terminated(RawGuzzleKeyedAttribute::parse)?;

            punctuated_attrs.into_iter().for_each(|attr| match attr {
                RawGuzzleKeyedAttribute::Keys(keys) => guzzle_attributes.keys = keys,
                RawGuzzleKeyedAttribute::Parser(parser) => guzzle_attributes.parser = Some(parser),
            });
        }
        Ok(guzzle_attributes)
    }
}

pub enum RawGuzzleKeyedAttribute {
    Keys(Keys),
    Parser(Expr),
}

impl Parse for RawGuzzleKeyedAttribute {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            match name_str.as_ref() {
                "keys" => Ok(RawGuzzleKeyedAttribute::Keys(input.parse()?)),
                "parser" => Ok(RawGuzzleKeyedAttribute::Parser(input.parse()?)),
                _ => Err(input.error(format!("Unknown key: {}", name_str))),
            }
        } else {
            Err(input.error("Attributes must be listed as `key = value`"))
        }
    }
}

/// Keys are a vector of `LitStr`, but we need to impl Parse for them, so we use a `newtype` and
/// impl `Deref` for when we want to see whats inside.
#[derive(Default)]
pub struct Keys(Vec<LitStr>);

impl Parse for Keys {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let content;
        bracketed!(content in input);
        let parser = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
        let parsed_keys = parser(&content).unwrap();
        Ok(Keys(parsed_keys.into_iter().collect()))
    }
}

impl Debug for Keys {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        // We can infer the span type.
        let strings: Vec<(String, _)> = self.iter().map(|s| (s.value(), s.span())).collect();
        for (string, span) in strings {
            writeln!(f, "{} - {:?}", string, span)?;
        }
        Ok(())
    }
}

impl Deref for Keys {
    type Target = Vec<LitStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attr::{RawGuzzleKeyedAttribute, GuzzleKeyedAttribute};
    use quote::quote;
    use syn::{parse::Parser, parse2, punctuated::Punctuated, LitStr, Token};

    #[test]
    fn parse_lit_str() {
        let token_stream = quote! { "single-key" };
        let lit_str: LitStr = parse2(token_stream).unwrap();
        assert_eq!("single-key".to_string(), lit_str.value());
    }

    #[test]
    fn parse_separated_lit_str() {
        let token_stream = quote! { "key1", "key2" };
        let parser = Punctuated::<LitStr, Token![,]>::parse_separated_nonempty;
        let punctuated_lit_str = parser.parse2(token_stream).unwrap();
        let mut iter = punctuated_lit_str.iter();
        assert_eq!("key1", iter.next().unwrap().value());
        assert_eq!("key2", iter.next().unwrap().value());
        assert!(iter.next().is_none());
    }

    #[test]
    fn parse_slice_lit_str() -> Result<(), syn::Error> {
        let token_stream = quote! { ["key1", "key2"] };
        let keys: Keys = parse2(token_stream).unwrap();
        let mut iter = keys.iter();
        assert_eq!("key1", &iter.next().unwrap().value());
        assert_eq!("key2", &iter.next().unwrap().value());
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn parse_named_slice() -> Result<(), syn::Error> {
        let token_stream = quote! { keys = ["key1", "key2"] };
        let attribute: RawGuzzleKeyedAttribute = parse2(token_stream).unwrap();
        let mut iter = match &attribute {
            RawGuzzleKeyedAttribute::Keys(keys) => keys.iter(),
            _ => panic!("attribute was not 'keys'"),
        };
        assert_eq!("key1", &iter.next().unwrap().value());
        assert_eq!("key2", &iter.next().unwrap().value());
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn parse_named_slices() -> Result<(), syn::Error> {
        let token_stream = quote! { ( keys = ["key1", "key2"], keys = ["key3", "key4"] ) };
        let attributes: GuzzleKeyedAttribute = parse2(token_stream).unwrap();
        let mut iter = attributes.keys.iter();
        assert_eq!("key3", &iter.next().unwrap().value());
        assert_eq!("key4", &iter.next().unwrap().value());
        assert!(iter.next().is_none());
        Ok(())
    }

    fn test_parser(s: String) -> String {
        s
    }

    #[test]
    fn parse_parser() -> Result<(), syn::Error> {
        let token_stream = quote! { parser = test_parser };
        let attribute: RawGuzzleKeyedAttribute = parse2(token_stream)?;
        Ok(())
    }
}
