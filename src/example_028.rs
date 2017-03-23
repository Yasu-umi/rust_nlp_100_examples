#!rust run

extern crate regex;
use self::regex::Regex;

mod lib;


fn main() {
    let re1 = Regex::new(r"'{2,5}").unwrap();
    let re2 = Regex::new(r"\[{2}([^|\]]+?\|)*(.+?)\]{2}").unwrap();
    let re3 = Regex::new(r"\{{2}.+?\|.+?\|(.+?)\}{2}").unwrap();
    let re4 = Regex::new(r"<.*?>").unwrap();
    let re5 = Regex::new(r"\[.*?\]").unwrap();
    let formatter = |t: String| {
        let mut tmp_t = (*re1.replace_all(t.as_str(), "")).to_string();
        tmp_t = (*re2.replace_all(tmp_t.as_str(), "$2")).to_string();
        tmp_t = (*re3.replace_all(tmp_t.as_str(), "$2")).to_string();
        tmp_t = (*re4.replace_all(tmp_t.as_str(), "")).to_string();
        tmp_t = (*re5.replace_all(tmp_t.as_str(), "")).to_string();
        tmp_t
    };
    let hash = lib::fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
