#!rust run

use std::env;

fn print_odd_text(text: &str) {
    let mut odd_text = String::new();
    for (idx, t) in text.chars().enumerate() {
        if idx % 2 == 0 {
            odd_text.push(t)
        }
    }
    println!("{}", &odd_text);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(text) => print_odd_text(text),
        None => println!("pass one argument"),
    }
}
