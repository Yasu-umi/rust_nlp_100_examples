#!rust run
#![feature(ord_max_min)]

extern crate nlp_100_examples;
extern crate rand;
extern crate serde;

use nlp_100_examples::*;
use rand::Rng;

use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");

    let min_window_size = 1;
    let max_window_size = 6;

    if !Path::new(&config.enwiki_corpus_path).exists() {
       println!("Please cargo run --bin 082");
       return
    }

    let f = fs::File::open(&config.enwiki_corpus_path)
        .expect(format!("Failed to open {}", &config.enwiki_corpus_path).as_str());
    let file = BufReader::new(&f);
    let raw_lines = file.lines()
        .collect::<Vec<_>>();
    let words_by_line = raw_lines
        .iter()
        .flat_map(|line| line)
        .filter(|line| !line.is_empty())
        .map(|line|
            line.split_whitespace()
                .filter(|word| !word.is_empty())
                .map(|word| word.to_owned())
                .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>();
    println!("create words_by_line");

    let words_map = words_by_line.iter()
        .flat_map(|words| words)
        .collect::<HashSet<&String, RandomState>>()
        .into_iter()
        .enumerate()
        .map(|(idx, word)| (word, idx))
        .collect::<HashMap<&String, usize>>();
    println!("create words_map");

    let window_sets_by_line = words_by_line.iter()
        .filter_map(|words| {
            let words_len = words.len();
            if words_len <= 1 {
                None
            } else {
                let window_set = words.iter()
                    .enumerate()
                    .map(|(word_pos, word)| {
                        let window_size = rand::thread_rng().gen_range(min_window_size, max_window_size);
                        let window_start = if window_size < word_pos { word_pos - window_size } else { 0 };
                        let window_end = if 1 < word_pos { word_pos - 1 } else { 0 };
                        let window = (window_start..window_end)
                            .chain((word_pos + 1).min(words_len)..(word_pos + window_size).min(words_len))
                            .map(|window_pos|
                                words_map.get(
                                    words.get(window_pos)
                                        .expect("can't find window word")
                                ).expect("can't find word")
                            )
                            .collect::<Vec<&usize>>();
                        (
                            words_map.get(&word).expect("can't find word"),
                            window
                        )
                    })
                    .collect::<Vec<(&usize, Vec<&usize>)>>();
                Some(window_set)
            }
        });

    let mut tc_counter: HashMap<(&usize, &usize), usize> = HashMap::new();
    let mut c_counter: HashMap<&usize, usize> = HashMap::new();
    let mut t_counter: HashMap<&usize, usize> = HashMap::new();
    let mut i = 0;
    for window_sets in window_sets_by_line {
        for window_set in window_sets.iter() {
            for window in window_set.1.iter() {
                *tc_counter.entry((&window_set.0, window)).or_insert(0) += 1;
                *t_counter.entry(&window_set.0).or_insert(0) += 1;
                *c_counter.entry(window).or_insert(0) += 1;
                i += 1;
                if i % 1000000 == 0 { println!("{}", i); }
            }
        }
    }

    let ti_counter = t_counter.into_iter()
        .enumerate()
        .map(|(i, (k, v))| (k, (v, i)))
        .collect::<HashMap<&usize, (usize, usize)>>();
    let ci_counter = c_counter.into_iter()
        .enumerate()
        .map(|(i, (k, v))| (k, (v, i)))
        .collect::<HashMap<&usize, (usize, usize)>>();

    bin_utils::dump(&config.words_map_bin_path, &words_map);
    bin_utils::dump(&config.tc_counter_bin_path, &tc_counter);
    bin_utils::dump(&config.ti_counter_bin_path, &ti_counter);
    bin_utils::dump(&config.ci_counter_bin_path, &ci_counter);

    println!("N = {}", i);
}
