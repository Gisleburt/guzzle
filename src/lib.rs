//! Guzzle
//! ======
//!
//! [![GitHub release](https://img.shields.io/github/release/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/releases)
//! [![GitHub license](https://img.shields.io/github/license/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/blob/master/LICENSE.md)
//! This project is an experimental work in progress and the interface may change.
//!
//! ToDo:
//! -----
//!
//! - [x] Parse types
//! - [x] Use attributes
//! - [x] Toggle off per attribute
//! - [ ] Recurse over objects that also implement the trait
//! - [ ] Open source and release
//!
//! What problem are we trying to solve?
//! ------------------------------------
//!
//! We're using a Wordpress database which contains non normalised data in "meta" tables. These tables give us data as a
//! set of keys and values that are attached to another object.
//!
//! For example, a single post in wp_posts may have multiple entries in wp_postmeta such as `location_lat`, `location_lng`,
//! `author`. We may wish to take those first two `location_*` keys and put the values into a `Location` struct, but leave
//! any other keys for use in different structs.
//!
//! A handy way to do this is to turn the meta data into iterator, then use a filter_map. For example:
//!
//! ```rust
//! #[derive(Default)]
//! struct Location {
//!     lat: String,
//!     lng: String,
//! }
//!
//! impl Location {
//!     fn guzzle(&mut self, (key, value): (String, String)) -> Option<(String, String)> {
//!         match key.as_ref() {
//!             "location_lat" => self.lat = value,
//!             "location_lng" => self.lng = value,
//!             _ => return Some((key, value)),
//!         };
//!         None
//!     }
//! }
//!
//! let metadata = vec![
//!     ("location_lat".to_string(), "51.5074° N".to_string()),
//!     ("location_lng".to_string(), "0.1278° W".to_string()),
//!     ("author".to_string(), "danielmason".to_string()),
//! ];
//!
//! let mut location = Location::default();
//! let left_overs = metadata.into_iter().filter_map(|data| location.guzzle(data));
//! ```
//!
//! However, we don't want to have to implement the same function over and over. Instead we can use the custom derive
//! `Guzzle`.
//!
//! ```rust
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     #[guzzle(keys = ["location_lat"])]
//!     lat: String,
//!     #[guzzle(keys = ["location_lng"])]
//!     lng: String,
//! }
//!
//! let metadata = vec![
//!     ("location_lat", "51.5074° N".to_string()),
//!     ("location_lng", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lat, "51.5074° N".to_string());
//! assert_eq!(location.lng, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```
//!
//! If your fields names are the same as your meta data keys, you do not need to specify them.
//!
//! ```rust
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     lat: String,
//!     lng: String,
//! }
//!
//! let metadata = vec![
//!     ("lat", "51.5074° N".to_string()),
//!     ("lng", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lat, "51.5074° N".to_string());
//! assert_eq!(location.lng, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```
//!
//! If you want to prevent a field from receiving data from guzzle, add `#[noguzzle]`
//!
//! ```rust
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     lat: String,
//!     #[noguzzle]
//!     lng: String,
//! }
//!
//! let metadata = vec![
//!     ("lat", "51.5074° N".to_string()),
//!     ("lng", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lat, "51.5074° N".to_string());
//!
//! assert_eq!(remaining_data, [
//!     ("lng", "0.1278° W".to_string()),
//!     ("some-other-key", "some-other-key".to_string())
//! ]);
//! ```
//!
//! If your data is not a string, you may provide a parser function
//!
//! ```rust
//! use guzzle::Guzzle;
//!
//! fn string_to_u64(s: String) -> f64 {
//!     s.parse().unwrap_or_default()
//! }
//!
//! #[derive(Default, Guzzle)]
//! struct Location {
//!     #[guzzle(keys = ["location_lat"], parser = string_to_u64)]
//!     lat: f64,
//!     #[guzzle(keys = ["location_lng"], parser = string_to_u64)]
//!     lng: f64,
//! }
//!
//! let metadata = vec![
//!     ("location_lat", "51.5074".to_string()),
//!     ("location_lng", "0.1278".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut location = Location::default();
//!
//! let remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//!
//! assert_eq!(location.lat, 51.5074);
//! assert_eq!(location.lng, 0.1278);
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
