#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().expect("Failed to load config");
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let mappings = mecab_utils::feature_mappings(text);

        for mapping in mappings {
            println!("{:?}", mapping);
        }
    }
}
