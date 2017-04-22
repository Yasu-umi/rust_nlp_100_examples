#!rust run

extern crate rand;
use std::env;
use rand::Rng;


fn shuffle_str(sentence: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut char_list = Vec::new();
    for one_char in sentence.to_string().chars() {
        char_list.push(one_char);
    }
    rng.shuffle(char_list.as_mut_slice());
    char_list.iter().map(|s| s.to_string()).fold(String::new(), |res, s| res + &s)
}

fn typoglycemia(sentence: &str) -> String {
    let words = sentence.split(' ');
    let mut shuffled_words = Vec::new();
    for word in words {
        if word.len() > 4 {
            let mut origin_word = word.to_string();
            let first_char = origin_word.remove(0 as usize);
            let last_char = origin_word.pop().unwrap();
            let suffled_chars = shuffle_str(origin_word.as_str());
            let mut tmp_word = String::new();
            tmp_word.push(first_char);
            let mut converted_word = tmp_word + suffled_chars.as_str();
            converted_word.push(last_char);
            shuffled_words.push(converted_word);
        } else {
            shuffled_words.push(word.to_string());
        }
    }
    shuffled_words.join(" ")
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(sentence) => {
            let shuffled_sentence = typoglycemia(sentence);
            println!("{}", shuffled_sentence)
        }
        _ => println!("specify 1 sentence"),
    }
}
