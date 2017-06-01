#!rust run

use std::env;

fn print_sorted_num_of_words(sentence: &str) {
    let words = sentence.split(" ")
        .map(|word| word.replace(".", "").replace(",", ""))
        .filter(|word| !word.is_empty());
    let mut i_words: Vec<(String, usize)> = words.enumerate()
        .map(|(i, word)| {
            let end = if [1, 5, 6, 7, 8, 9, 15, 16, 19].contains(&i) { 1 } else { 2 };
            (format!("{}", &word[0..end]), i)
        })
        .collect();
    i_words.sort_by_key(|tuple| tuple.clone().0);
    for (i, word) in i_words.into_iter() {
        println!("{} {}", i, word)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(sentence) => print_sorted_num_of_words(sentence),
        None => println!("pass one argument"),
    }
}
