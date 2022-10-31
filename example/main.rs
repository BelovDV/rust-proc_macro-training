extern crate utility;

use utility::FieldsFromStrings;

#[derive(FieldsFromStrings, Debug)]
struct Test {
    a: i32,
    b: i32,
}

fn main() {
    let mut t = Test { a: 1, b: 2 };

    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() == 1 {
                if words[0] == "quit" || words[0] == "q" {
                    break;
                }
            } else if words.len() == 3 {
                if words[0] == "set" {
                    t.set_from_string(words[1], words[2])
                        .unwrap_or_else(|e| println!("{e}"));
                    dbg!(&t);
                    continue;
                }
            }
            println!("cannot parse input");
        }
    }
}
