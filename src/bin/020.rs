#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let texts = fetch::country_texts("イギリス");
    for text in texts {
        println!("{:?}", text);
    }
}
