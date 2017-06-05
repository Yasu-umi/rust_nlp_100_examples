#!rust run

extern crate nlp_100_examples;

use std::collections::HashMap;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().expect("Failed to load config");
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let mappings = mecab_utils::feature_mappings(text);

        let mut counter: HashMap<String, u32> = HashMap::new();
        for mapping in mappings {
            let value = counter.entry(mapping.surface).or_insert(0);
            *value += 1;
        }
        let mut sorted_counter_tumple = counter.iter().collect::<Vec<(&String, &u32)>>();
        sorted_counter_tumple.sort_by(|&(_, value_a), &(_, value_b)| value_b.cmp(value_a));

        for (key, value) in sorted_counter_tumple {
            println!("{}: {}", key, value)
        }
    }
}
