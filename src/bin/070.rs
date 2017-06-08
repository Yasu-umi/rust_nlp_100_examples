#!rust run

extern crate nlp_100_examples;
extern crate encoding;
extern crate rand;

use nlp_100_examples::*;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;

use rand::{thread_rng, Rng};

use std::fs;
use std::io::{BufWriter, Write};


fn create_ng_pos_lines(files: Vec<(Vec<u8>, String)>) -> Vec<String> {
    let mut lines = Vec::new();
    for (text, path) in files {
        if let Ok(text) = ISO_8859_1.decode(text.as_slice(), DecoderTrap::Strict) {
            let prefix = if path.find("pos").is_some() {
                    Some("+1 ".to_owned())
                } else if path.find("neg").is_some() {
                    Some("-1 ".to_owned())
                } else { None };
            let tmp_lines_opt = prefix.map(|_prefix|
                text.split("\n")
                    .filter(|line| !line.is_empty()).map(|line| _prefix.clone() + line)
                    .collect::<Vec<String>>()
            );
            if let Some(mut tmp_lines) = tmp_lines_opt {
                lines.append(&mut tmp_lines);
            }
        }
    }
    lines
}

fn shuffle_vec<T>(mut vec: Vec<T>) -> Vec<T>
    where T: std::clone::Clone {
    let mut slice = vec.as_mut_slice();
    let mut rng = thread_rng();
    rng.shuffle(slice);
    slice.to_vec()
}


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");

    let client = fetch::create_client();
    let files = fetch::tar_gz_files(client, config.movie_review_data_url)
        .expect("Failed to fetch tar.gz");

    let lines = create_ng_pos_lines(files);
    let shuffled_lines = shuffle_vec(lines);

    let mut file = BufWriter::new(fs::File::create(config.sentiment_path).unwrap());
    for line in shuffled_lines {
        file.write(line.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
}