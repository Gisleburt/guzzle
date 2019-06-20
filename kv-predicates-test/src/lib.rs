#[cfg(test)]
#[macro_use]
extern crate kv_predicates_derive;

#[cfg(test)]
mod tests {
    use kv_predicates::KVPredicatesSimple;

    #[derive(KVPredicatesSimple)]
    struct Test {
        one: String,
        two: String,
    }

    #[test]
    fn it_works() {
        assert_eq!(Test::test(), "Hello, Macro! My name is Test");
    }
}
