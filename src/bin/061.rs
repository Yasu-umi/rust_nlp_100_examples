#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;
use std::env;

fn main() {
    if let Some(name) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(connect) = redis::create_connect("redis://127.0.0.1/") {
            if let Ok(area) = redis::get_area_by_name(&connect, name) {
                println!("{}", area)
            } else {
                println!("not found")
            }
        }
    } else {
        println!("pass name")
    }
}
