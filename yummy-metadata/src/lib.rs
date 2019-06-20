pub trait YummyMetadata {
    fn eat_yummy_metadata<T>(&mut self, current: (T, String)) -> Option<(T, String)>
    where
        T: AsRef<str>;
}

#[cfg(test)]
mod tests {
    use crate::YummyMetadata;

    #[derive(Default)]
    struct Tester {
        pub one: String,
        pub two: String,
    }

    impl YummyMetadata for Tester {
        fn eat_yummy_metadata<T>(&mut self, (key, value): (T, String)) -> Option<(T, String)>
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
    fn eat_yummy_metadata_with_vec_string_string() {
        let test_data = vec![
            ("one".to_string(), "1".to_string()),
            ("two".to_string(), "2".to_string()),
            ("three".to_string(), "3".to_string()),
        ];

        let mut tester = Tester::default();

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

    #[test]
    fn eat_yummy_metadata_with_vec_str_string() {
        let test_data = vec![
            ("one", "1".to_string()),
            ("two", "2".to_string()),
            ("three", "3".to_string()),
        ];

        let mut tester = Tester::default();

        let remaining_data: Vec<(&str, String)> = test_data
            .into_iter()
            .filter_map(|v| tester.eat_yummy_metadata(v))
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
    fn eat_yummy_metadata_with_hash_str_string() {
        use std::collections::HashMap;

        let test_data: HashMap<String, String> = vec![
            ("one".to_string(), "1".to_string()),
            ("two".to_string(), "2".to_string()),
            ("three".to_string(), "3".to_string()),
        ]
        .into_iter()
        .collect();

        let mut tester = Tester::default();

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
