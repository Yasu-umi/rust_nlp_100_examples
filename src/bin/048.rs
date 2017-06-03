#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let chunked_sentences = structs::ChunkedSentenceIter::from_sentences(text);

        for chunked_sentence in chunked_sentences {
            let len = chunked_sentence.len();
            for i in 0..len {
                if i == len - 1 { continue; }
                if let Some(chunk) = chunked_sentence.get(i) {
                    if chunk.has_noun() {
                        let path = chunk
                            .to_root_iter(&chunked_sentence)
                            .map(|chunk| chunk.surfaces())
                            .fold(String::new(), (|acc, item| {
                                if acc.len() == 0 { item } else { acc + " -> " + item.as_str() }
                            }));
                        println!("{}", path);
                    }
                }
            }
        }
    }
}
