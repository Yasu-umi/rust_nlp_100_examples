#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().expect("Failed to load config");
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let chunked_sentences = structs::ChunkedSentenceIter::from_sentences(text);
        for chunked_sentence in chunked_sentences {
            let tmp_chunked_sentence = chunked_sentence.clone();
            for chunk in chunked_sentence {
                if chunk.has_verb() {
                    let tmp_chunk = chunk.clone();
                    if let Some(verb) = tmp_chunk.morphs.get(0) {
                        let srcs = chunk.srcs;
                        let particles = srcs.into_iter()
                            .filter_map(|idx| tmp_chunked_sentence.get(idx))
                            .flat_map(structs::Chunk::morphs_of_particle)
                            .fold(String::new(), |acc, morph| {
                                let next = morph.base.clone();
                                if acc.len() > 0 { acc + " " + next.as_str() } else { next }
                            });
                        println!("{}\t{}", verb.base, particles);
                    }
                }
            }
        }
    }
}
