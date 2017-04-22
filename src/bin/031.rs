#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let input = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let mappings = mecab_utils::feature_mappings(input);

    let mut surfaces: Vec<String> = Vec::new();
    for mapping in mappings {
        if mapping.pos == "動詞" {
            surfaces.push(mapping.surface);
        }
    }

    for surface in surfaces {
        println!("{}", surface)
    }
}
