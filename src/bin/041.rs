#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let chunked_sentences = structs::Chunk::from_sentences(text);

        println!("{:?}", chunked_sentences.get(2).unwrap());
    }
}
