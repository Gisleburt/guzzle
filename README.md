Yummy Metadata
==============

ToDo:
-----

- [ ] Check types
- [ ] Use attributes instead of getting all fields
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
    fn eat_yummy_metadata(&mut self, (key, value): (String, String)) -> Option<(String, String)> {
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
    ("location_lat".to_string(), "51.5074".to_string())
    ("location_lng".to_string(), "0.1278".to_string())
    ("author".to_string(), "danielmason".to_string())
];

let location = Location::default();
let left_overs = metadata.into_iter().filter_map(|data| location.eat_yummy_metadata(data));
```

However, we don't want to have to implement the same function over and over. Instead we can use the custom derive
`YummyMetadata`.

```rust
#[derive(YummyMetadata)]
struct Location {
    #[meta_key(location_lat)]
    lat: String,
    #[meta_key(location_lng)]
    lng: String,
}
```
