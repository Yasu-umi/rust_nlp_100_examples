#!rust run

extern crate regex;
use self::regex::Regex;

mod lib;


fn main() {
    let re = Regex::new(r"'{2,5}").unwrap();
    let formatter = |t: String| (*re.replace_all(t.as_str(), "")).to_string();
    let hash = lib::fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
