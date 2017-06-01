#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};

fn save_text_as_graph(filedir: &str) {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let chunked_sentences = structs::Chunk::from_sentences(text.to_string());

    for (i, chunked_sentence) in chunked_sentences.iter().enumerate() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for (j, chunk) in chunked_sentence.iter().enumerate() {
            nodes.push((j, chunk.surfaces()));
            if let Some(dst) = chunk.dst {
                edges.push((j, dst));
            }
        }

        let filepath = Path::new(filedir).join(format!("{}.dot", i));
        if let Ok(file) = File::create(filepath) {
            let mut writer = BufWriter::new(file);

            writer.write("digraph {\n    graph [charset=\"UTF-8\"];".as_bytes()).unwrap();
            for (j, node) in nodes {
                writer.write(format!("\n    N{}[label=\"{}\"]", j, node).as_bytes()).unwrap();
            }
            for (j, dst) in edges {
                writer.write(format!("\n    N{} -> N{}[label=\"\"]", j, dst).as_bytes()).unwrap();
            }
            writer.write("\n}".as_bytes()).unwrap();
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(filedir) = args.get(1) {
        save_text_as_graph(filedir);
    } else {
        println!("pass filedir");
    }
}
