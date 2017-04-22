#!rust run

use std::env;

fn print_sorted_num_of_words(sentence: &str) {
    let len_list = sentence.split(' ')
        .filter(|word| !word.is_empty())
        .map(|word| word.len())
        .collect::<Vec<_>>();
    println!("{}", len_list.len());
    for len in len_list {
        println!("{}", len);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(sentence) => print_sorted_num_of_words(sentence),
        None => println!("pass one argument"),
    }
}
