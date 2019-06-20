#[cfg(test)]
#[macro_use]
extern crate yummy_metadata_derive;

#[cfg(test)]
mod tests {
    use yummy_metadata::YummyMetadata;

    #[derive(Default, Debug, YummyMetadata)]
    struct PredicateTester {
        one: String,
        two: String,
    }

    #[test]
    fn it_works() {
        let test_data = vec![
            ("one".to_string(), "1".to_string()),
            ("two".to_string(), "2".to_string()),
            ("three".to_string(), "3".to_string()),
        ];

        let mut tester = PredicateTester::default();

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
