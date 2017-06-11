extern crate encoding;
extern crate regex;
extern crate wordnet_stemmer;

use self::encoding::{Encoding, DecoderTrap};
use self::encoding::all::ISO_8859_1;
use self::wordnet_stemmer::{WordnetStemmer};
use self::regex::Regex;

use wordnet_utils;

use std::collections::HashMap;


pub fn create_lines_from_latin1<'a, T>(raw_texts: T)
    -> Vec<String>
    where T: Iterator<Item=Vec<u8>> + 'a {
    raw_texts
        .filter_map(|raw_text|
            ISO_8859_1.decode(raw_text.as_slice(), DecoderTrap::Strict).map(|text|
                text.split("\n")
                    .filter(|line| !line.is_empty())
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            ).ok()
        )
        .fold(Vec::new(), |mut acc, mut item| {
            acc.append(&mut item);
            acc
        })
}

pub fn create_word_counter_from_lines(lines: &Vec<String>)
    -> HashMap<String, u32> {
    let re = Regex::new(r"[,.:;-\\)\\(\?\s]").unwrap();
    let mut counter = HashMap::new();
    for line in lines {
        for term in re.split(line.as_str()) {
            if !term.is_empty() {
                let value = counter.entry(term.to_owned()).or_insert(0);
                *value += 1;
            }
        }
    }
    counter
}

pub fn sorted_by_frequent_terms_from_lines(lines: &Vec<String>)
    -> impl Iterator<Item=String> {
    let counter = create_word_counter_from_lines(lines);
    let mut count_vec: Vec<(String, u32)> = counter.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec.into_iter().map(|(term, _)| term)
}

pub fn get_features_from_line<'a, T>(wn: &'a WordnetStemmer, lines: T, stop_words: Vec<String>)
    -> impl Iterator<Item=Vec<(String, Option<wordnet_utils::Part>)>> + 'a
    where T: Iterator<Item=&'a String> + 'a {
    let re = Regex::new(r"[,.:;-\\)\\(\?\s]").unwrap();
    lines.map(move |line|
        re.split(line.as_str())
            .filter(|&term| !term.is_empty() && !stop_words.contains(&term.to_owned().to_lowercase()))
            .map(|term| wordnet_utils::lemma(wn, term.to_owned()))
            .collect()
    )
}
