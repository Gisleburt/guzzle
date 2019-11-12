use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct GuzzleExample {
    /// This field is annotated with no_guzzle, therefore it will not be parsed by guzzle
    #[no_guzzle]
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

// This is the deeply nested `TypeThatAlsoImplementsGuzzle`
#[derive(Default, Guzzle)]
struct TypeThatAlsoImplementsGuzzle {
    /// This struct represents some deeply nested data
    #[guzzle(keys = ["deep_data"], parser = bool_parser)]
    deeply_nested_data: bool
}

// These are the parsers referenced above
fn u64_parser(s: String) -> u64 {
    s.parse().unwrap()
}
fn bool_parser(s: String) -> bool {
    s.parse().unwrap()
}

fn main() {
    // These are our keys and values
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

    // Create our object
    let mut guzzle_example = GuzzleExample::default();

    // Feed our keys and values to our object, capturing any that weren't consumed
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
