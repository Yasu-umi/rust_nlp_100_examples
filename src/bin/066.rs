#!rust run

extern crate nlp_100_examples;
#[macro_use(doc, bson)]
extern crate bson;

use nlp_100_examples::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(area) => {
            let config = config::Config::new()
                .expect("Failed to load config");

            let collection = mongo_utils::create_collection(&config)
                .expect("Failed to initialize mongo client");

            let count = collection.count(Some(doc! { "name" => area }), None)
                .ok().expect("Failed to execute count.");
            println!("{}", count);
        },
        None => println!("pass area"),
    }
}