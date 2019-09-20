//! This crate defines Guzzle and its custom derive.
//!
//! Example:
//! ```
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     #[guzzle(keys = ["lng"])]
//!     lng: String,
//!     #[guzzle(keys = ["lat"])]
//!     lat: String,
//! }
//!
//! let test_data = vec![
//!     ("lng", "51.5074° N".to_string()),
//!     ("lat", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = test_data
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lng, "51.5074° N".to_string());
//! assert_eq!(location.lat, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```
//! However, the names of your keys may not match your struct names. To map those values, use the
//! guzzle attribute macro:
//! ```
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     #[guzzle(keys = ["longitude", "lng"])]
//!     lng: String,
//!     #[guzzle(keys = ["latitude", "lat"])]
//!     lat: String,
//! }
//!
//! let test_data = vec![
//!     ("longitude", "51.5074° N".to_string()),
//!     ("lat", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = test_data
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lng, "51.5074° N".to_string());
//! assert_eq!(location.lat, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```

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

        #[derive(Default, Guzzle)]
        struct AttributeDemo {
            /// This field is not annotated, therefore its field is `basic` and its keys contain
            /// one string which is the same as the name `basic`.
            basic: String,
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
            /// This field will be ignored
            #[noguzzle]
            ignored_field: String,
        }

        #[test]
        fn everything() {
            use std::collections::HashMap;

            fn my_parser(s: String) -> u64 {
                s.parse().unwrap_or(0)
            }

            let test_data: Vec<(&str, String)> = vec![
                ("basic", "basic info".to_string()),
                ("one", "1".to_string()),
                ("two", "2".to_string()),
                ("other_types", "20".to_string()),
                ("three", "3".to_string()),
                ("four", "4".to_string()),
                ("ignored_field", "ignored data".to_string()),
            ];

            let mut attribute_demo = AttributeDemo::default();

            let remaining_data: Vec<(&str, String)> = test_data
                .into_iter()
                .filter_map(|v| attribute_demo.guzzle(v))
                .collect();

            //            assert_eq!(attribute_demo.basic, "basic info".to_string());
            assert_eq!(attribute_demo.listed_keys, "2".to_string());
            assert_eq!(attribute_demo.other_types, 20);
            assert_eq!(attribute_demo.other_types_with_listed_keys, 4);
            assert!(attribute_demo.ignored_field.is_empty());
        }
    }
}
