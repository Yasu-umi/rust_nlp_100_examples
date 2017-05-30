#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let chunked_sentences = structs::Chunk::from_sentences(text);

    for chunked_sentence in chunked_sentences {
        let len = chunked_sentence.len();
        for i in 0..len {
            if i == len - 1 { continue; }
            if let Some(chunk) = chunked_sentence.get(i) {
                let path_vec = chunk.to_root(chunked_sentence.iter().collect());
                let path = path_vec.iter()
                    .map(|chunk| chunk.surfaces())
                    .fold(String::new(), (|acc, item| {
                        if acc.len() == 0 { item } else { acc + " -> " + item.as_str() }
                    }));
                println!("{}", path);
            }
        }
    }
}
