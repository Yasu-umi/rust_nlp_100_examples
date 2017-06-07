#!rust run

extern crate nlp_100_examples;
#[macro_use(doc, bson)]
extern crate bson;

use nlp_100_examples::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(name) => {
            let config = config::Config::new()
                .expect("Failed to load config");

            let collection = mongo_utils::create_collection(&config)
                .expect("Failed to initialize mongo client");

            let query = doc! { "aliases.name" => name };
            let cursor = mongo_utils::create_artist_cursor(&collection, Some(query))
                .ok().expect("Failed to execute find.");
            for artist in cursor.filter_map(|item| item) {
                println!("{:?}", artist);
            }
        },
        None => println!("pass name"),
    }
}