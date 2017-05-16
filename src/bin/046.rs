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
          let particles = chunk.srcs.clone().into_iter()
            .filter_map(|idx| tmp_chunked_sentence.get(idx))
            .flat_map(structs::Chunk::morphs_of_particle)
            .fold(String::new(), |acc, morph| { acc + "\t" + morph.base.clone().as_str() });
          let morphs = chunk.srcs.clone().into_iter()
            .map(|idx| tmp_chunked_sentence.get(idx))
            .map(|opt_chunk| opt_chunk
              .map(|chunk| chunk.morphs.iter().fold(String::new(), |acc, morph| { acc + morph.surface.as_str() }))
            )
            .fold(String::new(), |acc, surfaces| { acc + "\t" + surfaces.unwrap_or(String::new()).as_str() });
          if verb.is_some() {
            println!("{}{}{}", verb.unwrap().base, particles, morphs);
          }
        }
      }
    }
}
