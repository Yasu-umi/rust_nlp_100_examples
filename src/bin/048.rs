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
            let mut path_vec = Vec::new();
            if let Some(mut chunk) = chunked_sentence.get(i) {
                path_vec.push(chunk.surfaces());
                while let Some(dst) = chunk.dst {
                    if let Some(dst_chunk) = chunked_sentence.get(dst) {
                        path_vec.push(dst_chunk.surfaces());
                        chunk = dst_chunk;
                    }
                }
            }
            let path = path_vec.iter().fold(String::new(), (|acc, item| {
                if acc.len() == 0 { item.to_string() } else { acc + " -> " + item }
            }));
            println!("{}", path);
        }
    }
}
