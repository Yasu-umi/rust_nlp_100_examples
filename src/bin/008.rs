#!rust run

use std::env;


fn atbash_cipher(sentence: &str) -> String {
    let mut dec_sentence = String::new();
    let lower = "abcdefghijklmnopqrstuvwxyz";
    for string in sentence.chars() {
        if lower.contains(string) {
            let code = 219 - (string as u8);
            dec_sentence.push(code as char);
        } else {
            dec_sentence.push(string);
        }
    }
    dec_sentence
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(sentence) => {
            let dec_sentence = atbash_cipher(sentence);
            println!("{}", dec_sentence)
        }
        _ => println!("specify 1 sentence"),
    }
}
