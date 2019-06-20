#[cfg(test)]
#[macro_use]
extern crate kv_predicates_derive;

#[cfg(test)]
mod tests {
    use kv_predicates::KVPredicates;

    #[derive(Default, Debug, KVPredicates)]
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
            .filter_map(|v| tester.filter_map_predicate(v))
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
