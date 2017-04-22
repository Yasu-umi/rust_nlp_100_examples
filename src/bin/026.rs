#!rust run

extern crate regex;
extern crate nlp_100_examples;

use self::regex::Regex;

use nlp_100_examples::*;


fn main() {
    let re = Regex::new(r"'{2,5}").unwrap();
    let formatter = |t: String| (*re.replace_all(t.as_str(), "")).to_string();
    let hash = fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
