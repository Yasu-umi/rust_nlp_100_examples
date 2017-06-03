#!rust run

extern crate regex;
extern crate nlp_100_examples;

use regex::Regex;
use nlp_100_examples::*;


fn main() {
    let re = Regex::new(r"(File|ファイル):(.*\.[a-zA-Z0-9]+)\|.*").unwrap();
    if let Ok(config) = config::Config::new() {
        if let Ok(texts) = fetch::country_texts(config.country_json_url.as_str(), "イギリス") {
            let lines = texts
                .filter_map(|l|
                    if let Some(caps) = re.captures(l.as_str()) {
                        Some(caps[2].to_string())
                    } else {
                        None
                    }
                );
            for line in lines {
                println!("{:?}", line);
            }
        }
    }
}
