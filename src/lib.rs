//! Guzzle
//! ======
//!
//! [![GitHub release](https://img.shields.io/github/release/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/releases)
//! [![GitHub license](https://img.shields.io/github/license/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/blob/master/LICENSE.md)
//!
//! What is is?
//! -----------
//!
//! Guzzle is a Custom Derive that will help you pass a stream of keys and values into a struct.
//!
//! Usage
//! -----
//!
//! Derive Guzzle, then use the attribute tags to annotate your struct
//!
//! ```rust
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct GuzzleExample {
//!     /// This field is annotated with no_guzzle, therefore it will not be parsed by guzzle
//!     #[no_guzzle]
//!     ignored: String,
//!
//!     /// This field is not annotated, so if a key matches the field name, it will set the value
//!     basic: String,
//!
//!     /// This field is annotated, but with no keys, so it uses the field name as a key
//!     /// Currently `#[guzzle]` doesn't do anything different to the default behavior (see
//!     /// above), but in the future the default behaviour may be toggleable.
//!     #[guzzle]
//!     basic_too: String,
//!
//!     /// This field may be filled from multiple keys
//!     #[guzzle(keys = ["one", "two"])]
//!     listed_keys: String,
//!
//!     /// This field is not a string, you must provider a parser that will transform it into
//!     /// the correct type
//!     #[guzzle(parser = u64_parser)]
//!     other_types: u64,
//!
//!     /// This field isn't a string and has multiple keys
//!     #[guzzle(parser = u64_parser, keys = ["three", "four"])]
//!     other_types_with_listed_keys: u64,
//!
//!     /// Guzzle will wire up this field so that data being guzzled by the `GuzzleExample`
//!     /// will first be sent to the `TypeThatAlsoImplementsGuzzle`. If the
//!     /// `TypeThatAlsoImplementsGuzzle` consumes the value, `GuzzleExample` will not.
//!     #[deep_guzzle]
//!     recurse_guzzle_to_populate_this_field: TypeThatAlsoImplementsGuzzle,
//! }
//!
//! // This is the deeply nested `TypeThatAlsoImplementsGuzzle`
//! #[derive(Default, Guzzle)]
//! struct TypeThatAlsoImplementsGuzzle {
//!     /// This struct represents some deeply nested data
//!     #[guzzle(keys = ["deep_data"], parser = bool_parser)]
//!     deeply_nested_data: bool
//! }
//!
//! // These are the parsers referenced above
//! fn u64_parser(s: String) -> u64 {
//!     s.parse().unwrap()
//! }
//! fn bool_parser(s: String) -> bool {
//!     s.parse().unwrap()
//! }
//!
//! // These are our keys and values
//! let example_data: Vec<(&str, String)> = vec![
//!     ("basic", "basic info".to_string()),
//!     ("basic_too", "more basic info".to_string()),
//!     ("one", "1".to_string()),
//!     ("two", "2".to_string()),
//!     ("other_types", "20".to_string()),
//!     ("three", "3".to_string()),
//!     ("four", "4".to_string()),
//!     ("ignored", "ignored data".to_string()),
//!     ("deep_data", "true".to_string())
//! ];
//!
//! // Create our object
//! let mut guzzle_example = GuzzleExample::default();
//!
//! // Feed our keys and values to our object, capturing any that weren't consumed
//! let remaining_data: Vec<(&str, String)> = example_data
//!     .into_iter()
//!     .filter_map(|v| guzzle_example.guzzle(v))
//!     .collect();
//!
//! // All appropriate fields are now set
//! assert_eq!(guzzle_example.basic, "basic info".to_string());
//! assert_eq!(guzzle_example.basic_too, "more basic info".to_string());
//! assert_eq!(guzzle_example.listed_keys, "2".to_string());
//! assert_eq!(guzzle_example.other_types, 20);
//! assert_eq!(guzzle_example.other_types_with_listed_keys, 4);
//!
//! // Including the deeply nested field
//! assert!(guzzle_example.recurse_guzzle_to_populate_this_field.deeply_nested_data);
//!
//! // Ignored data is left over
//! assert!(guzzle_example.ignored.is_empty());
//! assert_eq!(remaining_data, vec![("ignored", "ignored data".to_string())]);
//! ```
//!
//! Example Use Case
//! ----------------
//!
//! This project came out of a need to take Wordpress metadata data and put it into complex Rust
//! structs.
//!

pub use guzzle_derive::*;

pub trait Guzzle {
    fn guzzle<T>(&mut self, current: (T, String)) -> Option<(T, String)>
    where
        T: AsRef<str>;
}

#[cfg(test)]
mod tests {
    mod guzzle_trait {
        use crate::Guzzle;

        #[derive(Default)]
        struct Tester {
            pub one: String,
            pub two: String,
        }

        impl Guzzle for Tester {
            fn guzzle<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
            where
                T: AsRef<str>,
            {
                match key.as_ref() {
                    "one" => self.one = value,
                    "two" => self.two = value,
                    _ => return Some((key, value)),
                }
                None
            }
        }

        #[test]
        fn guzzle_with_vec_string_string() {
            let test_data = vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
                ("three".to_string(), "3".to_string()),
            ];

            let mut tester = Tester::default();

            let remaining_data: Vec<(String, String)> = test_data
                .into_iter()
                .filter_map(|v| tester.guzzle(v))
                .collect();

            assert_eq!(tester.one, "1".to_string());
            assert_eq!(tester.two, "2".to_string());

            assert_eq!(remaining_data.len(), 1);
            assert_eq!(
                remaining_data.into_iter().next(),
                Some(("three".to_string(), "3".to_string()))
            );
        }

        #[test]
        fn guzzle_with_vec_str_string() {
            let test_data = vec![
                ("one", "1".to_string()),
                ("two", "2".to_string()),
                ("three", "3".to_string()),
            ];

            let mut tester = Tester::default();

            let remaining_data: Vec<(&str, String)> = test_data
                .into_iter()
                .filter_map(|v| tester.guzzle(v))
                .collect();

            assert_eq!(tester.one, "1".to_string());
            assert_eq!(tester.two, "2".to_string());

            assert_eq!(remaining_data.len(), 1);
            assert_eq!(
                remaining_data.into_iter().next(),
                Some(("three", "3".to_string()))
            );
        }

        #[test]
        fn guzzle_with_hash_str_string() {
            use std::collections::HashMap;

            let test_data: HashMap<String, String> = vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
                ("three".to_string(), "3".to_string()),
            ]
            .into_iter()
            .collect();

            let mut tester = Tester::default();

            let remaining_data: Vec<(String, String)> = test_data
                .into_iter()
                .filter_map(|v| tester.guzzle(v))
                .collect();

            assert_eq!(tester.one, "1".to_string());
            assert_eq!(tester.two, "2".to_string());

            assert_eq!(remaining_data.len(), 1);
            assert_eq!(
                remaining_data.into_iter().next(),
                Some(("three".to_string(), "3".to_string()))
            );
        }
    }

    mod guzzle_meta_data_derive {
        use crate::Guzzle;

        fn u64_parser(s: String) -> u64 {
            s.parse().unwrap()
        }
        fn bool_parser(s: String) -> bool {
            s.parse().unwrap()
        }

        #[derive(Default, Guzzle)]
        struct TypeThatAlsoImplementsGuzzle {
            /// This struct represents some deeply nested data
            #[guzzle(keys = ["deep_data"], parser = bool_parser)]
            deeply_nested_data: bool,
        }

        #[derive(Default, Guzzle)]
        struct GuzzleExample {
            /// This field is annotated with no_guzzle, therefore it will not be parsed by guzzle
            #[no_guzzle]
            ignored: String,

            /// This field is not annotated, so if a key matches the field name, it will set the
            /// value
            basic: String,

            /// This field is annotated, but with no keys, so it uses the field name as a key
            /// Currently `#[guzzle]` doesn't do anything different to the default behavior (see
            /// above), but in the future the default behaviour may be toggleable.
            #[guzzle]
            basic_too: String,

            /// This field may be filled from multiple keys
            #[guzzle(keys = ["one", "two"])]
            listed_keys: String,

            /// This field is not a string, you must provider a parser that will transform it into
            /// the correct type
            #[guzzle(parser = u64_parser)]
            other_types: u64,

            /// This field isn't a string and has multiple keys
            #[guzzle(parser = u64_parser, keys = ["three", "four"])]
            other_types_with_listed_keys: u64,

            /// Guzzle will wire up this field so that data being guzzled by the `GuzzleExample`
            /// will first be sent to the `TypeThatAlsoImplementsGuzzle`. If the
            /// `TypeThatAlsoImplementsGuzzle` consumes the value, `GuzzleExample` will not.
            #[deep_guzzle]
            recurse_guzzle_to_populate_this_field: TypeThatAlsoImplementsGuzzle,
        }

        #[test]
        fn everything() {
            let test_data: Vec<(&str, String)> = vec![
                ("basic", "basic info".to_string()),
                ("basic_too", "more basic info".to_string()),
                ("one", "1".to_string()),
                ("two", "2".to_string()),
                ("other_types", "20".to_string()),
                ("three", "3".to_string()),
                ("four", "4".to_string()),
                ("ignored", "ignored data".to_string()),
                ("deep_data", "true".to_string()),
            ];

            let mut guzzle_example = GuzzleExample::default();

            // This just makes sure deeply_nested_data isn't accidentally set to true
            assert!(
                !guzzle_example
                    .recurse_guzzle_to_populate_this_field
                    .deeply_nested_data
            );

            let remaining_data: Vec<(&str, String)> = test_data
                .into_iter()
                .filter_map(|v| guzzle_example.guzzle(v))
                .collect();

            // All appropriate fields are now set
            assert_eq!(guzzle_example.basic, "basic info".to_string());
            assert_eq!(guzzle_example.basic_too, "more basic info".to_string());
            assert_eq!(guzzle_example.listed_keys, "2".to_string());
            assert_eq!(guzzle_example.other_types, 20);
            assert_eq!(guzzle_example.other_types_with_listed_keys, 4);

            // Including the deeply nested field
            assert!(
                guzzle_example
                    .recurse_guzzle_to_populate_this_field
                    .deeply_nested_data
            );

            // Ignored data is left over
            assert!(guzzle_example.ignored.is_empty());
            assert_eq!(
                remaining_data,
                vec![("ignored", "ignored data".to_string())]
            );
        }
    }

    mod try_build {
        use trybuild::TestCases;

        #[test]
        fn everything() {
            let test_case = TestCases::new();
            test_case.pass("tests/passing/*.rs");
            test_case.compile_fail("tests/failing/*.rs");
        }
    }
}
