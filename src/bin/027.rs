#!rust run

extern crate regex;
extern crate nlp_100_examples;

use self::regex::Regex;

use nlp_100_examples::*;


fn main() {
    let re1 = Regex::new(r"'{2,5}").unwrap();
    let re2 = Regex::new(r"\[{2}([^|\]]+?\|)*(.+?)\]{2}").unwrap();
    let formatter = |t: String| {
        let tmp_t = (*re1.replace_all(t.as_str(), "")).to_string();
        (*re2.replace_all(tmp_t.as_str(), "$2")).to_string()
    };
    if let Ok(config) = config::Config::new() {
        if let Ok(hash) = fetch::get_template_hash(config.country_json_url.as_str(), "イギリス", formatter) {
            for (key, value) in hash {
                println!("{}: {}", key, value);
            }
        }
    }
}
