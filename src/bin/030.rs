#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let input = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let mappings = mecab_utils::feature_mappings(input);

    for mapping in mappings {
        println!("{}", mapping);
    }
}
