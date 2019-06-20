pub trait KVPredicates {
    fn filter_map_predicate<T>(&mut self, current: (T, String)) -> Option<(T, String)>
    where
        T: AsRef<str>;
}

pub trait KVPredicatesSimple {
    fn test() -> String;
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
        fn filter_map_predicate<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
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
    fn filter_map_predicate_with_vec_string_string() {
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

    #[test]
    fn filter_map_predicate_with_vec_str_string() {
        let test_data = vec![
            ("one", "1".to_string()),
            ("two", "2".to_string()),
            ("three", "3".to_string()),
        ];

        let mut tester = PredicateTester::default();

        let remaining_data: Vec<(&str, String)> = test_data
            .into_iter()
            .filter_map(|v| tester.filter_map_predicate(v))
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
    fn filter_map_predicate_with_hash_str_string() {
        use std::collections::HashMap;

        let test_data: HashMap<String, String> = vec![
            ("one".to_string(), "1".to_string()),
            ("two".to_string(), "2".to_string()),
            ("three".to_string(), "3".to_string()),
        ]
        .into_iter()
        .collect();

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
