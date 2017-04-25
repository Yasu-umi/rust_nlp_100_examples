#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let morphed_sentences = structs::Morph::from_sentences(text);

    println!("{:?}", morphed_sentences.get(2).unwrap());
}
