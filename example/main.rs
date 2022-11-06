pub mod tests;

use reflection_proc::*;

#[derive(FieldsFromStrings, Debug, Default)]
struct Test {
    a: i32,
    b: i32,
    string: String,
    double: f64,
}

fn main() {
    let mut t: Test = Default::default();

    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() == 0 {
                continue;
            }
            match words[0] {
                "quit" | "q" => break,
                "set" | "s" if words.len() == 3 => {
                    t.set_from_string(words[1], words[2])
                        .unwrap_or_else(|e| println!("{e}, wrong key?"));
                }
                "set" | "s" => {
                    println!("error, expected `set key value`")
                }
                "get" | "g" => println!("{:#?}", &t),
                "help" | "h" => {
                    println!("possible commands: q,s,g,h");
                    println!("  better look into source, it's easer");
                }
                _ => println!("cannot parse input"),
            }
        }
    }
}
