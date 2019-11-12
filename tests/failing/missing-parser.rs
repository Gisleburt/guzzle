use guzzle::Guzzle;

#[derive(Default, Guzzle)]
struct GuzzleExample {
    /// This parser is not defined so we should see an error for it
    #[guzzle(parser = u64_parser)]
    other_types: u64,
}

fn main() {
    // These are our keys and values
    let test_data: Vec<(&str, String)> = vec![
        ("other_types", "20".to_string()),
    ];

    // Create our object
    let mut guzzle_example = GuzzleExample::default();

    // Feed our keys and values to our object, capturing any that weren't consumed
    let remaining_data: Vec<(&str, String)> = test_data
        .into_iter()
        .filter_map(|v| guzzle_example.guzzle(v))
        .collect();

    // All appropriate fields are now set
    assert_eq!(guzzle_example.other_types, 20);
}
