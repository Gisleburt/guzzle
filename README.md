Guzzle
======

ToDo:
-----

- [ ] Check types
- [x] Use attributes instead of getting all fields
- [ ] Recurse over objects that also implement the trait
- [ ] Open source and release

What problem are we trying to solve?
------------------------------------

We're using a Wordpress database which contains non normalised data in "meta" tables. These tables give us data as a
set of keys and values that are attached to another object.

For example, a single post in wp_posts may have multiple entries in wp_postmeta such as `location_lat`, `location_lng`,
`author`. We may wish to take those first two `location_*` keys and put the values into a `Location` struct, but leave
any other keys for use in different structs.

A handy way to do this is to turn the meta data into iterator, then use a filter_map. For example:

```rust
struct Location {
    lat: String,
    lng: String,
}

impl Location {
    fn guzzle(&mut self, (key, value): (String, String)) -> Option<(String, String)> {
        match key.as_ref() {
            "location_lat" => self.lat = value,
            "location_lng" => self.lng = value,
            _ => return Some(key, value);
        };
        None
    }
}
```

Used like this:

```rust
let metadata = vec![
    ("location_lat".to_string(), "51.5074° N".to_string())
    ("location_lng".to_string(), "0.1278° W".to_string())
    ("author".to_string(), "danielmason".to_string())
];

let location = Location::default();
let left_overs = metadata.into_iter().filter_map(|data| location.guzzle(data));
```

However, we don't want to have to implement the same function over and over. Instead we can use the custom derive
`Guzzle`.

```rust
#[derive(Guzzle)]
struct Location {
    #[guzzle(keys = ["location_lat"])]
    lat: String,
    #[guzzle(keys = ["location_lng"])]
    lng: String,
}

let metadata = vec![
    ("location_lat".to_string(), "51.5074° N".to_string())
    ("location_lng".to_string(), "0.1278° W".to_string())
    ("some-other-key".to_string(), "some-other-key".to_string())
];

let remaining_data: Vec<(&str, String)> = test_data
    .into_iter()
    .filter_map(|v| location.guzzle(v))
    .collect();

assert_eq!(location.lng, "51.5074° N".to_string());
assert_eq!(location.lat, "0.1278° W".to_string());

assert_eq!(remaining_data, [("some-other-key", "some-other-key".to_string())]);
```
