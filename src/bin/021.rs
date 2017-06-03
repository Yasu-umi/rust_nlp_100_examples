#!rust run

extern crate regex;
extern crate nlp_100_examples;

use regex::Regex;

use nlp_100_examples::*;

fn main() {
    let re = Regex::new(".*Category.*").unwrap();
    if let Ok(config) = config::Config::new() {
        if let Ok(texts) = fetch::country_texts(config.country_json_url.as_str(), "イギリス") {
            let lines = texts
                .filter(|l| re.is_match(l));
            for line in lines {
                println!("{}", line);
            }
        }
    }
}
