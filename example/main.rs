pub mod tests;

use reflection::*;

#[derive(Debug, Default, Reflection, UpdateWithStr)]
struct Inner {
    abc: String,
    d: f64,
}

#[derive(Debug, Default, Reflection, UpdateWithStr)]
struct Test {
    a: i32,
    b: i32,
    string: String,
    double: f64,

    // rec: Box<Option<Test>>,
    inner: Inner,
}

fn main() {
    let mut t: Test = Default::default();

    println!();
    dbg!(&t);
    println!();
    dbg!(Test::get_field_list());
    dbg!(&t);
    t.update_with_str("inner abc d").unwrap();
    t.update_with_str("string abc").unwrap();
    dbg!(&t);
}
