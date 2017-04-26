#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let chunked_sentences = structs::Chunk::from_sentences(text);

    for chunked_sentence in chunked_sentences {
      let tmp_chunked_sentence = chunked_sentence.clone();
      for chunk in chunked_sentence {
        match chunk.dst.map(|dst| tmp_chunked_sentence.get(dst).unwrap()) {
          Some(dst_chunk) => println!("{}\t{}", chunk.surfaces(), dst_chunk.surfaces()),
          _ => (),
        }
      }
    }
}
