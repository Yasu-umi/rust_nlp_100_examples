#!rust run

extern crate nlp_100_examples;
use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let mappings = mecab_utils::feature_mappings(text);

        let mut nouns: Vec<String> = Vec::new();
        let mut noun_mappings: Vec<structs::Morph> = Vec::new();
        for mapping in mappings {
            if mapping.pos == "名詞" {
                noun_mappings.push(mapping);
            } else {
                let len = noun_mappings.len();
                if len > 1 {
                    let noun = noun_mappings.iter()
                        .map(|mapping| mapping.surface.clone())
                        .collect::<Vec<String>>()
                        .concat();
                    nouns.push(noun);
                }
                if len > 0 {
                    noun_mappings = Vec::new();
                }
            }
        }

        for noun in nouns {
            println!("{}", noun)
        }
    }
}
