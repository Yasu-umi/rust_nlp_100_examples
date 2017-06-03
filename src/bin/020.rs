#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    if let Ok(config) = config::Config::new() {
        if let Ok(texts) = fetch::country_texts(config.country_json_url.as_str(), "イギリス") {
            for text in texts {
                println!("{}", text);
            }
        }
    }
}
