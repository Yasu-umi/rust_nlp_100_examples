#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let input = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let mappings = mecab_utils::feature_mappings(input);

    let mut bases: Vec<String> = Vec::new();
    for mapping in mappings {
        if mapping.pos == "動詞" {
            bases.push(mapping.base);
        }
    }

    for base in bases {
        println!("{}", base)
    }
}
