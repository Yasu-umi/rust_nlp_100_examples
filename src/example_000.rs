#!rust run

use std::env;

fn print_rev_text(text: &str) {
    let rev_text = text.chars().rev().collect::<String>();
    println!("{}", &rev_text);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(text) => print_rev_text(text),
        None => println!("pass one argument"),
    }
}
