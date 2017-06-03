#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let chunked_sentences = structs::ChunkedSentenceIter::from_sentences(text);

        for chunked_sentence in chunked_sentences {
            let tmp_chunked_sentence = chunked_sentence.clone();
            for chunk in chunked_sentence {
                if chunk.has_noun() {
                    match chunk.dst.map(|dst| tmp_chunked_sentence.get(dst).unwrap()) {
                        Some(dst_chunk) => {
                            if dst_chunk.has_verb() {
                                println!("{}\t{}", chunk.surfaces(), dst_chunk.surfaces());
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
