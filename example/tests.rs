#[cfg(test)]
mod tests {
    use reflection::*;

    #[test]
    fn test_struct_fields_set() {
        #[derive(FieldsFromStrings, Default)]
        struct TestS {
            a: i32,
            b: i32,
            string: String,
            double: f64,
        }

        let mut t: TestS = Default::default();

        assert!(t.set_from_string("a", "5") == Ok(()));
        assert!(t.a == 5);

        assert!(t.set_from_string("double", "34.43") == Ok(()));
        assert!(t.double == 34.43);

        assert!(t.set_from_string("abc", "def") == Err("Wrong field name 'abc'".to_string()));

        assert!(t.set_from_string("a", "b") == Err("invalid digit found in string".to_string()));
    }

    #[derive(EnumFromStrings, PartialEq)]
    enum TestE {
        A,
        B,
        Abc,
    }

    #[test]
    fn test_enum_new() {
        use std::str::FromStr;
        assert!(Ok(TestE::A) == TestE::from_str("A"));
        assert!(Ok(TestE::B) == TestE::from_str("B"));
        assert!(Ok(TestE::Abc) == TestE::from_str("Abc"));
        assert!(Err("wrong string") == TestE::from_str("Abcd"));
        assert!(Err("wrong string") == TestE::from_str("a"));
    }

    #[derive(Debug, ReflectionString, Default, UpdateWithStr)]
    struct TestRSSInner {
        s: String,
        ss: i32,
    }
    #[derive(Debug, ReflectionString, Default, UpdateWithStr)]
    struct TestRSS {
        inner: TestRSSInner,
        a: i32,
        abc_d: f64,
    }
    // #[derive(ReflectionString)]
    // enum TestRSE {} // TODO: implement for enum.

    #[test]
    fn test_reflection_string() {
        assert_eq!(TestRSS::get_field_list(), vec!["inner", "a", "abc_d"]);

        let mut t: TestRSS = Default::default();
        assert_eq!(t.inner.s, "".to_string());

        // TODO: this shouldn't be Err.
        assert_eq!(t.get_field_string("inner ss"), Err(()));

        assert_eq!(t.get_field_string("a"), Ok("0".to_string()));

        assert_eq!(t.update_with_str("inner ss 12"), Ok(()));

        assert_eq!(t.inner.get_field_string("ss"), Ok("12".to_string()));
    }
}
