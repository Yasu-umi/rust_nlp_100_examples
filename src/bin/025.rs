#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    fn formatter(t: String) -> String {
        t
    };
    let hash = fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
