#!rust run

extern crate regex;
use self::regex::Regex;

mod lib;


fn main() {
    let re1 = Regex::new(r"'{2,5}").unwrap();
    let re2 = Regex::new(r"\[{2}([^|\]]+?\|)*(.+?)\]{2}").unwrap();
    let formatter = |t: String| {
        let tmp_t = (*re1.replace_all(t.as_str(), "")).to_string();
        (*re2.replace_all(tmp_t.as_str(), "$2")).to_string()
    };
    let hash = lib::fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
