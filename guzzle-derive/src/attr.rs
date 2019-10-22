use std::default::Default;
use std::fmt::{Debug, Error as FormatError, Formatter};
use std::ops::Deref;
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseBuffer},
    punctuated::Punctuated,
    Expr, Ident, LitStr, Token,
};

#[derive(Default)]
pub struct GuzzleAttributes {
    pub keys: Keys,
    pub parser: Option<Expr>,
}

impl GuzzleAttributes {
    pub fn new() -> GuzzleAttributes {
        GuzzleAttributes::default()
    }

    pub fn set_default_key_if_none(&mut self, ident: Ident) {
        if self.keys.is_empty() {
            self.keys = Keys(vec![LitStr::new(ident.to_string().as_str(), ident.span())])
        }
    }
}

impl Parse for GuzzleAttributes {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let punctuated_attrs: Punctuated<GuzzleAttribute, Token![,]> =
            content.parse_terminated(GuzzleAttribute::parse)?;

        let mut guzzle_attributes = GuzzleAttributes::default();
        punctuated_attrs.into_iter().for_each(|attr| match attr {
            GuzzleAttribute::Keys(keys) => guzzle_attributes.keys = keys,
            GuzzleAttribute::Parser(parser) => guzzle_attributes.parser = Some(parser),
        });
        Ok(guzzle_attributes)
    }
}

pub enum GuzzleAttribute {
    Keys(Keys),
    Parser(Expr),
}

impl Parse for GuzzleAttribute {
    fn parse(input: &ParseBuffer) -> Result<Self, syn::Error> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            match name_str.as_ref() {
                "keys" => Ok(GuzzleAttribute::Keys(input.parse()?)),
                "parser" => Ok(GuzzleAttribute::Parser(input.parse()?)),
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
    use crate::attr::{GuzzleAttribute, GuzzleAttributes};
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
        let attribute: GuzzleAttribute = parse2(token_stream).unwrap();
        let mut iter = match &attribute {
            GuzzleAttribute::Keys(keys) => keys.iter(),
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
        let attributes: GuzzleAttributes = parse2(token_stream).unwrap();
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
        let attribute: GuzzleAttribute = parse2(token_stream)?;
        Ok(())
    }
}
