extern crate mecab;

use std::collections::HashMap;

use structs::*;


pub fn feature_mappings(input: String) -> Vec<Morph> {
    let mut tagger = mecab::Tagger::new("");
    let mut features: Vec<Morph> = Vec::new();
    for node in tagger.parse_to_node(input).iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => {}
            mecab::MECAB_EOS_NODE => {}
            _ => features.push(Morph::from_mecab_node(&node))
        }
    }
    features
}

pub fn get_words_sorted_by_freq(input: String) -> Vec<(String, u32)> {
    let mappings = feature_mappings(input);
    let mut counter: HashMap<String, u32> = HashMap::new();
    for mapping in mappings {
        let value = counter.entry(mapping.surface).or_insert(0);
        *value += 1;
    }
    let mut sorted_tumple_vec: Vec<(String, u32)> = counter.iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect();
    sorted_tumple_vec.sort_by(|&(_, value_a), &(_, value_b)| value_b.cmp(&value_a));
    sorted_tumple_vec
}

pub fn get_freq_words_vec(input: String) -> Vec<(u32, Vec<String>)> {
    let sorted_tumple_vec = get_words_sorted_by_freq(input);
    let mut map: HashMap<u32, Vec<String>> = HashMap::new();
    for (word, num) in sorted_tumple_vec {
        let words = map.entry(num).or_insert(Vec::new());
        words.push(word);
    }
    let mut vec: Vec<(u32, Vec<String>)> = map.iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect();
    vec.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    vec
}
