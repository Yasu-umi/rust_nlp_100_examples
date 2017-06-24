#!rust run

extern crate nlp_100_examples;
extern crate regex;
extern crate rand;
extern crate bincode;

use nlp_100_examples::*;
use rand::Rng;
use bincode::{serialize_into, Infinite};

use std::fs;
use std::path::Path;
use std::io::{BufRead, BufReader, BufWriter};
use std::collections::HashMap;


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

    let window_sets_by_line = words_by_line.iter()
        .map(|words|
            (0..words.len())
                .map(|word_pos| {
                    let window_size = rand::thread_rng().gen_range(min_window_size, max_window_size);
                    let window_start = if window_size < word_pos { word_pos - window_size } else { 0 };
                    let window_end = if 1 < word_pos { word_pos - 1 } else { 0 };
                    (
                        word_pos,
                        (window_start..window_end)
                            .chain((word_pos + 1)..(word_pos + window_size))
                            .collect::<Vec<usize>>()
                    )
                })
                .collect::<Vec<(usize, Vec<usize>)>>()
        );

    let mut tc_counter: HashMap<(&String, &String), u32> = HashMap::new();
    let mut c_counter: HashMap<&String, u32> = HashMap::new();
    let mut t_counter: HashMap<&String, u32> = HashMap::new();
    let mut i = 0;
    for (window_sets, words) in window_sets_by_line.zip(words_by_line.iter()) {
        for window_set in window_sets.iter() {
            let word = words.get(window_set.0)
                .expect("can't get word");
            for window in window_set.1.iter().filter_map(|pos| words.get(*pos)) {
                *tc_counter.entry((word, window)).or_insert(0) += 1;
                *t_counter.entry(word).or_insert(0) += 1;
                *c_counter.entry(window).or_insert(0) += 1;
                i += 1;
                if i % 1000000 == 0 { println!("{}", i); }
            }
        }
    }

    println!("N = {}", i);

    let f_tc_counter = fs::File::create(&config.tc_counter_bin_path)
        .expect("Failed to create tc_counter.bin");
    let mut w_tc_counter = BufWriter::new(&f_tc_counter);

    let f_t_counter = fs::File::create(&config.t_counter_bin_path)
        .expect("Failed to create tc_counter.bin");
    let mut w_t_counter = BufWriter::new(&f_t_counter);

    let f_c_counter = fs::File::create(&config.c_counter_bin_path)
        .expect("Failed to create tc_counter.bin");
    let mut w_c_counter = BufWriter::new(&f_c_counter);

    serialize_into(&mut w_tc_counter, &tc_counter, Infinite)
        .expect("Failed to dump tc_counter");
    println!("dump tc_counter");
    serialize_into(&mut w_t_counter, &t_counter, Infinite)
        .expect("Failed to dump t_counter");
    println!("dump t_counter");
    serialize_into(&mut w_c_counter, &c_counter, Infinite)
        .expect("Failed to dump c_counter");
    println!("dump c_counter");
}