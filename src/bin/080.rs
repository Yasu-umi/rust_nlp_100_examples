#!rust run

extern crate nlp_100_examples;
extern crate regex;

use nlp_100_examples::*;
use regex::Regex;

use std::fs;
use std::io::{BufWriter, Write};


fn main() {
    let client = fetch::create_client();
    let config = config::Config::new()
        .expect("Failed to load config");
    let bytes = fetch::read_urls(&client, config.enwiki_corpus_urls)
        .expect("Failed to fetch bz2");
    let text = fetch::bzip2_to_string(&bytes[..])
        .expect("Failed to parse bz2");

    let re = Regex::new("[[:punct:]]|['()]")
        .expect("Failed to compile regex");

    let mut f = BufWriter::new(fs::File::create(config.enwiki_corpus_path)
        .expect("Failed to parse bz2"));

    for line in text.split("\n").filter(|line| !line.is_empty()) {
        let mut sentence = line.split(" ")
            .map(|word| word.trim_matches(|c: char| re.is_match(c.to_string().as_str())))
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        sentence += "\n";
        f.write(sentence.as_bytes())
            .expect("Failed to write buf");
    }
}