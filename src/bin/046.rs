#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let chunked_sentences = structs::Chunk::from_sentences(text);

    for chunked_sentence in chunked_sentences {
      let tmp_chunked_sentence = chunked_sentence.clone();
      for chunk in chunked_sentence {
        if chunk.has_verb() {
          let tmp_chunk = chunk.clone();
          let verb = tmp_chunk.morphs.get(0);
          if verb.is_some() {
            let particles = chunk.srcs.clone().into_iter()
              .filter_map(|idx| tmp_chunked_sentence.get(idx))
              .flat_map(structs::Chunk::morphs_of_particle)
              .fold(String::new(), |acc, morph| { acc + "\t" + morph.base.as_str() });
            let morphs = chunk.srcs.clone().into_iter()
              .map(|idx| tmp_chunked_sentence.get(idx).map(|chunk| chunk.surfaces()))
              .fold(String::new(), |acc, surfaces| { acc + "\t" + surfaces.unwrap_or(String::new()).as_str() });
            println!("{}{}{}", verb.unwrap().base, particles, morphs);
          }
        }
      }
    }
}
