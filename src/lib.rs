//! Guzzle
//! ======
//!
//! [![GitHub release](https://img.shields.io/github/release/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/releases)
//! [![GitHub license](https://img.shields.io/github/license/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/blob/master/LICENSE.md)
//! This project is an experimental work in progress and the interface may change.
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
//!     fn guzzle<T: AsRef<str>>(&mut self, (key, value): (T, String)) -> Option<(T, String)> {
//!         match key.as_ref() {
//!             "lat" => self.lat = value,
//!             "lng" => self.lng = value,
//!             _ => return Some((key, value)),
//!         };
//!         None
//!     }
//! }
//!
//! let metadata = vec![
//!     ("lat", "51.5074° N".to_string()),
//!     ("lng", "0.1278° W".to_string()),
//!     ("author", "danielmason".to_string()),
//! ];
//!
//! let mut location = Location::default();
//! let _remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| location.guzzle(v))
//!     .collect();
//! ```
//!
//! However, we don't want to have to implement the same function over and over, that's why we created Guzzle. Instead we can use the custom derive
//!
//! Using Guzzle
//! ------------
//!
//! If your metadata happens to have keys that match your structs field names, all you need to do is `#[derive(Guzzle)]`.
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
//! assert_eq!(&location.lat, "51.5074° N");
//! assert_eq!(&location.lng, "0.1278° W");
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```
//!
//! Often times though, your metadata keys won't match your structs fields. For this, you can use the `#[guzzle]` field
//! attribute to list with keys should be used.
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
//! assert_eq!(&location.lat, "51.5074° N");
//! assert_eq!(&location.lng, "0.1278° W");
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
//! assert_eq!(&location.lat, "51.5074° N");
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
//!
//! Another problem with metadata (particularly Wordpress metdata) is that it's flat. Take our location for example, if it
//! was part of a users metadata, we may not want to apply it directly to a `User` struct, but rather to a `Location`
//! struct, which is part of a `User`.
//!
//! Unfortunately, Rust macro's can not yet gather information about the Type they are opperating on, so we can not simply
//! observe that a field _also_ "impls Guzzle", we have to tell the derive that information ourselves. We do this with the
//! `deep_guzzle` attribute.
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
//! #[derive(Default, Guzzle)]
//! struct User {
//!     // Optionally you can use #[guzzle], currently this behaves as if you didn't tag it but
//!     // in the future you may be able to toggle the default behavior so this will be used then
//!     #[guzzle]
//!     name: String,
//!     #[deep_guzzle]
//!     location: Location,
//! }
//!
//! let metadata = vec![
//!     ("lat", "51.5074° N".to_string()),
//!     ("lng", "0.1278° W".to_string()),
//!     ("name", "Robert Tables".to_string()),
//!     ("some-other-key", "some-other-key".to_string()),
//! ];
//!
//! let mut user = User::default();
//!
//! let remaining_data: Vec<(&str, String)> = metadata
//!     .into_iter()
//!     .filter_map(|v| user.guzzle(v))
//!     .collect();
//!
//! assert_eq!(&user.name, "Robert Tables");
//! assert_eq!(&user.location.lat, "51.5074° N");
//! assert_eq!(&user.location.lng, "0.1278° W");
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
        fn bool_parser(s: String) -> bool {
            s.parse().unwrap()
        }

        #[derive(Default, Guzzle)]
        struct TypeThatAlsoImplementsGuzzle {
            /// This struct represents some deeply nested data
            #[guzzle(keys = ["deep_data"], parser = bool_parser)]
            deeply_nested_data: bool
        }

        #[derive(Default, Guzzle)]
        struct GuzzleExample {
            /// This field is annotated with noguzzle, therefore it will not be parsed by guzzle
            #[noguzzle]
            ignored: String,

            /// This field is not annotated, so if a key matches the field name, it will set the value
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
                ("deep_data", "true".to_string())
            ];

            let mut guzzle_example = GuzzleExample::default();

            // This just makes sure deeply_nested_data isn't accidentally set to true
            assert!(!guzzle_example.recurse_guzzle_to_populate_this_field.deeply_nested_data);

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
            assert!(guzzle_example.recurse_guzzle_to_populate_this_field.deeply_nested_data);

            // Ignored data is left over
            assert!(guzzle_example.ignored.is_empty());
            assert_eq!(remaining_data, vec![("ignored", "ignored data".to_string())]);
        }
    }
}
