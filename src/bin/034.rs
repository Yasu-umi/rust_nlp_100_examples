#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let mappings = mecab_utils::feature_mappings(text);

        let mut nouns: Vec<String> = Vec::new();
        for idx in 0..(mappings.len() - 3) {
            let mapping_0 = mappings.get(idx).unwrap();
            let mapping_1 = mappings.get(idx + 1).unwrap();
            let mapping_2 = mappings.get(idx + 2).unwrap();
            if mapping_0.pos == "名詞" && mapping_1.surface == "の" && mapping_2.pos == "名詞" {
                let mut surface = mapping_0.surface.clone();
                surface = surface + mapping_1.surface.as_str();
                surface = surface + mapping_2.surface.as_str();
                nouns.push(surface);
            }
        }

        for noun in nouns {
            println!("{}", noun)
        }
    }
}
