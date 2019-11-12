use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct GuzzleExample {
    /// These keys should be string literals.
    #[guzzle(keys = [one, two])]
    listed_keys: String,
}

fn main() {
    // These are our keys and values
    let test_data: Vec<(&str, String)> = vec![
        ("one", "1".to_string()),
        ("two", "2".to_string()),
    ];

    // Create our object
    let mut guzzle_example = GuzzleExample::default();

    // Feed our keys and values to our object, capturing any that weren't consumed
    let remaining_data: Vec<(&str, String)> = test_data
        .into_iter()
        .filter_map(|v| guzzle_example.guzzle(v))
        .collect();

    // All appropriate fields are now set
    assert_eq!(guzzle_example.listed_keys, "2".to_string());
}
