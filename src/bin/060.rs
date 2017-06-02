#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

fn main() {
    println!("{:?}", redis::set_artists("redis://127.0.0.1/"));
}
