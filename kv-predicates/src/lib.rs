pub trait KVPredicates {
    fn filter_map_predicate(&mut self, current: (String, String)) -> Option<(String, String)>;
}

#[cfg(test)]
mod tests {
    use crate::KVPredicates;

    #[derive(Default)]
    struct PredicateTester {
        pub one: String,
        pub two: String,
    }

    impl KVPredicates for PredicateTester {
        fn filter_map_predicate(
            &mut self,
            (key, value): (String, String),
        ) -> Option<(String, String)> {
            match key.as_ref() {
                "one" => self.one = value,
                "two" => self.two = value,
                _ => return Some((key, value)),
            }
            None
        }
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
