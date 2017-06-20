extern crate encoding;
extern crate regex;
extern crate wordnet_stemmer;

use self::encoding::{Encoding, DecoderTrap};
use self::encoding::all::ISO_8859_1;
use self::wordnet_stemmer::{WordnetStemmer};
use self::regex::Regex;

use wordnet_utils;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;


pub fn create_lines_from_latin1<'a, T>(raw_texts: T)
    -> impl Iterator<Item=String> + 'a
    where T: Iterator<Item=&'a Vec<u8>> + 'a {
    raw_texts
        .filter_map(|raw_text|
            ISO_8859_1.decode(raw_text.as_slice(), DecoderTrap::Strict)
                .map(|text|
                    text.split("\n")
                        .filter(|line| !line.is_empty())
                        .map(|line| line.to_owned())
                        .collect::<Vec<String>>()
                )
                .ok()
        )
        .flat_map(|lines| lines)
}

pub fn create_word_counter_from_lines<'a, T>(lines: T)
    -> HashMap<String, u32>
    where T: Iterator<Item=&'a String> + 'a {
    let re = Regex::new(r"[,.:;-\\)\\(\?\s]").unwrap();
    let mut counter = HashMap::new();
    let terms = lines.flat_map(|line| {
            re.split(line.as_str())
                .filter(|term| !term.is_empty())
                .map(|term| term.to_owned())
                .peekable()
        });
    for term in terms {
        let value = counter.entry(term).or_insert(0u32);
        *value += 1u32;
    }
    counter
}

pub fn sorted_by_frequent_terms_from_lines<'a, T>(lines: T)
    -> Vec<(String, u32)>
    where T: Iterator<Item=&'a String> + 'a {
    let counter = create_word_counter_from_lines(lines);
    let mut count_vec: Vec<(String, u32)> = counter.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec
}

pub fn get_features_from_lines<'a, T>(wn: &'a WordnetStemmer, lines: T, stop_words: &'a HashSet<String, RandomState>)
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

pub fn get_stop_words<'a, T>(lines: T)
    -> impl Iterator<Item=String> + 'a
    where T: Iterator<Item=&'a String> + 'a {
     sorted_by_frequent_terms_from_lines(lines)
        .into_iter()
        .enumerate()
        .filter(|&(i, (ref word, count))| i < 100 || word.len() < 3 || count < 2)
        .map(|(_, (word, _))| word)
}
