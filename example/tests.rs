use reflection_proc::*;

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
