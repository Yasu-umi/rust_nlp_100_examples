#!rust run

extern crate cabocha;
extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let mut parser = cabocha::Parser::new("");
    let tree = parser.parse_to_tree(text);

    let mut morphed_sentences = Vec::new();
    let mut morphed_sentence = Vec::new();

    let mut i = 0;
    while morphed_sentences.len() < 3 {
        let morph = structs::Morph::from_cabocha_token(tree.token(i).unwrap());
        let is_end_of_sentence = morph.is_end_of_sentence();
        morphed_sentence.push(morph);
        if is_end_of_sentence {
            morphed_sentences.push(morphed_sentence);
            morphed_sentence = Vec::new();
        }
        i += 1;
    }
    println!("{:?}", morphed_sentences.last().unwrap());
}
