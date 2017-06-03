#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    fn formatter(t: String) -> String { t };
    if let Ok(config) = config::Config::new() {
        if let Ok(hash) = fetch::get_template_hash(config.country_json_url.as_str(), "イギリス", formatter) {
            for (key, value) in hash {
                println!("{}: {}", key, value);
            }
        }
    }
}
