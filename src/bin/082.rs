#!rust run

extern crate nlp_100_examples;
extern crate regex;
extern crate rand;

use nlp_100_examples::*;
use rand::Rng;

use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");

    let min_window_size = 1;
    let max_window_size = 6;

    if !Path::new(&config.enwiki_corpus_path).exists() {
       println!("Please cargo run --bin 081");
       return
    }

    let f = fs::File::open(&config.enwiki_corpus_path)
        .expect(format!("Failed to open {}", &config.enwiki_corpus_path).as_str());
    let file = BufReader::new(&f);
    for window_sets in file.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let line_vec = line.split_whitespace()
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>();
            line_vec.iter()
                .enumerate()
                .map(|(i, word)| {
                    let window_size = rand::thread_rng().gen_range(min_window_size, max_window_size);
                    (
                        word.to_string(),
                        ((if window_size < i { i - window_size } else { 0 })..(if 1 < i { i - 1 } else { 0 }))
                            .chain((i + 1)..(i + window_size))
                            .filter_map(|j| line_vec.get(j))
                            .map(|window_word| window_word.to_string())
                            .collect::<Vec<_>>()
                    )
                })
                .collect::<Vec<_>>()
        }) {
        for window_set in window_sets.iter() {
            for window in window_set.1.iter() {
                println!("{}\t{}", window_set.0, window);
            }
        }
    }
}