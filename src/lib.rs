//! This crate defines YummyMetadata and its custom derive.
//!
//! Example:
//! ```
//! use yummy_metadata::YummyMetadata;
//!
//! #[derive(Default, YummyMetadata)]
//! struct Location {
//!     lng: String,
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
//!     .filter_map(|v| location.eat_yummy_metadata(v))
//!     .collect();
//!
//! assert_eq!(location.lng, "51.5074° N".to_string());
//! assert_eq!(location.lat, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```
//! However, the names of your keys may not match your struct names. To map those values, use the
//! yummy attribute macro:
//! ```
//! use yummy_metadata::YummyMetadata;
//!
//! #[derive(Default, YummyMetadata)]
//! struct Location {
//!     #[yummy(longitude, lng)]
//!     lng: String,
//!     #[yummy(latitude, lat)]
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
//!     .filter_map(|v| location.eat_yummy_metadata(v))
//!     .collect();
//!
//! assert_eq!(location.lng, "51.5074° N".to_string());
//! assert_eq!(location.lat, "0.1278° W".to_string());
//!
//! assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
//! ```

pub use yummy_metadata_derive::*;

pub trait YummyMetadata {
    fn eat_yummy_metadata<T>(&mut self, current: (T, String)) -> Option<(T, String)>
    where
        T: AsRef<str>;
}

#[cfg(test)]
mod tests {
    mod yummy_meta_data_trait {
        use crate::YummyMetadata;

        #[derive(Default)]
        struct Tester {
            pub one: String,
            pub two: String,
        }

        impl YummyMetadata for Tester {
            fn eat_yummy_metadata<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
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
        fn eat_yummy_metadata_with_vec_string_string() {
            let test_data = vec![
                ("one".to_string(), "1".to_string()),
                ("two".to_string(), "2".to_string()),
                ("three".to_string(), "3".to_string()),
            ];

            let mut tester = Tester::default();

            let remaining_data: Vec<(String, String)> = test_data
                .into_iter()
                .filter_map(|v| tester.eat_yummy_metadata(v))
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
        fn eat_yummy_metadata_with_vec_str_string() {
            let test_data = vec![
                ("one", "1".to_string()),
                ("two", "2".to_string()),
                ("three", "3".to_string()),
            ];

            let mut tester = Tester::default();

            let remaining_data: Vec<(&str, String)> = test_data
                .into_iter()
                .filter_map(|v| tester.eat_yummy_metadata(v))
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
        fn eat_yummy_metadata_with_hash_str_string() {
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
                .filter_map(|v| tester.eat_yummy_metadata(v))
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

}
