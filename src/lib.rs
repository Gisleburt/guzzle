//! This crate defines Guzzle and its custom derive.
//!
//! Example:
//! ```
//! use guzzle::Guzzle;
//!
//! #[derive(Default, Guzzle)]
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
//!     #[guzzle("longitude", "lng")]
//!     lng: String,
//!     #[guzzle("latitude", "lat")]
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

}
