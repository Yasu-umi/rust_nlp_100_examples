#!rust run
#![feature(conservative_impl_trait)]

extern crate nlp_100_examples;
extern crate encoding;
extern crate regex;


use nlp_100_examples::*;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;

use regex::Regex;

use std::collections::HashMap;


fn create_lines(files: Vec<(Vec<u8>, String)>) -> Vec<String> {
    let mut lines = Vec::new();
    for (text, _) in files {
        if let Ok(text) = ISO_8859_1.decode(text.as_slice(), DecoderTrap::Strict) {
            lines.append(
                &mut text.split("\n")
                    .filter(|line| !line.is_empty())
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            );
        }
    }
    lines
}

fn create_word_counter_from_lines(lines: &Vec<String>) -> HashMap<String, u32> {
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

fn sorted_by_frequent_terms_from_lines(lines: &Vec<String>)
    -> impl Iterator<Item=String> {
    let counter = create_word_counter_from_lines(lines);
    let mut count_vec: Vec<(String, u32)> = counter.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec.into_iter().map(|(term, _)| term)
}


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");
    let client = fetch::create_client();
    let all_files = fetch::tar_gz_files(client, config.movie_review_data_url)
        .expect("Failed to fetch tar.gz");
    let files = all_files.into_iter()
        .filter(|&(_, ref path)| path.find("pos").is_some() || path.find("neg").is_some())
        .collect();
    let lines = create_lines(files);
    let terms = sorted_by_frequent_terms_from_lines(&lines);
    for (i, term) in terms.enumerate() {
        println!("{}", term);
        if i == 99 { break; }
    }
}