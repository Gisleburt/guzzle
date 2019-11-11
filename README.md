Guzzle
======

[![GitHub release](https://img.shields.io/github/release/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/releases)
[![GitHub license](https://img.shields.io/github/license/apolitical/guzzle.svg)](https://github.com/apolitical/guzzle/blob/master/LICENSE.md)
This project is an experimental work in progress and the interface may change.

What problem are we trying to solve?
------------------------------------

We're using a Wordpress database which contains non normalised data in "meta" tables. These tables give us data as a
set of keys and values that are attached to another object.

For example, a single post in wp_posts may have multiple entries in wp_postmeta such as `location_lat`, `location_lng`,
`author`. We may wish to take those first two `location_*` keys and put the values into a `Location` struct, but leave
any other keys for use in different structs.

A handy way to do this is to turn the meta data into iterator, then use a filter_map. For example:

```rust
#[derive(Default)]
struct Location {
    lat: String,
    lng: String,
}

impl Location {
    fn guzzle<T: AsRef<str>>(&mut self, (key, value): (T, String)) -> Option<(T, String)> {
        match key.as_ref() {
            "lat" => self.lat = value,
            "lng" => self.lng = value,
            _ => return Some((key, value)),
        };
        None
    }
}

let metadata = vec![
    ("lat", "51.5074° N".to_string()),
    ("lng", "0.1278° W".to_string()),
    ("author", "danielmason".to_string()),
];

let mut location = Location::default();
let remaining_data: Vec<(&str, String)> = metadata
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();
```

However, we don't want to have to implement the same function over and over, that's why we created Guzzle. Instead we can use the custom derive

Using Guzzle
------------

If your metadata happens to have keys that match your structs field names, all you need to do is `#[derive(Guzzle)]`.

```rust
use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct Location {
    lat: String,
    lng: String,
}

let metadata = vec![
    ("lat", "51.5074° N".to_string()),
    ("lng", "0.1278° W".to_string()),
    ("some-other-key", "some-other-key".to_string()),
];

let mut location = Location::default();

let remaining_data: Vec<(&str, String)> = metadata
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();

assert_eq!(location.lat, "51.5074° N".to_string());
assert_eq!(location.lng, "0.1278° W".to_string());

assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
```

Often times though, your metadata keys won't match your structs fields. For this, you can use the `#[guzzle]` field
attribute to list with keys should be used.

```rust
use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct Location {
    #[guzzle(keys = ["location_lat"])]
    lat: String,
    #[guzzle(keys = ["location_lng"])]
    lng: String,
}

let metadata = vec![
    ("location_lat", "51.5074° N".to_string()),
    ("location_lng", "0.1278° W".to_string()),
    ("some-other-key", "some-other-key".to_string()),
];

let mut location = Location::default();

let remaining_data: Vec<(&str, String)> = metadata
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();

assert_eq!(location.lat, "51.5074° N".to_string());
assert_eq!(location.lng, "0.1278° W".to_string());

assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
```

If you want to prevent a field from receiving data from guzzle, add `#[noguzzle]`

```rust
use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct Location {
    lat: String,
    #[noguzzle]
    lng: String,
}

let metadata = vec![
    ("lat", "51.5074° N".to_string()),
    ("lng", "0.1278° W".to_string()),
    ("some-other-key", "some-other-key".to_string()),
];

let mut location = Location::default();

let remaining_data: Vec<(&str, String)> = metadata
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();

assert_eq!(location.lat, "51.5074° N".to_string());

assert_eq!(remaining_data, [
    ("lng", "0.1278° W".to_string()),
    ("some-other-key", "some-other-key".to_string())
]);
```

If your data is not a string, you may provide a parser function

```rust
use guzzle::Guzzle;

fn string_to_u64(s: String) -> f64 {
    s.parse().unwrap_or_default()
}

#[derive(Default, Guzzle)]
struct Location {
    #[guzzle(keys = ["location_lat"], parser = string_to_u64)]
    lat: f64,
    #[guzzle(keys = ["location_lng"], parser = string_to_u64)]
    lng: f64,
}

let metadata = vec![
    ("location_lat", "51.5074".to_string()),
    ("location_lng", "0.1278".to_string()),
    ("some-other-key", "some-other-key".to_string()),
];

let mut location = Location::default();

let remaining_data: Vec<(&str, String)> = metadata
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();

assert_eq!(location.lat, 51.5074);
assert_eq!(location.lng, 0.1278);

assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
```