#!rust run

extern crate nlp_100_examples;
#[macro_use(doc, bson)]
extern crate bson;
extern crate mongodb;

use nlp_100_examples::*;
use mongodb::coll::options::FindOptions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(tag) => {
            let config = config::Config::new()
                .expect("Failed to load config");

            let collection = mongo_utils::create_collection(&config)
                .expect("Failed to initialize mongo client");

            let query = doc! { "tags.value" => tag };
            let mut option = FindOptions::new();
            option.sort = Some(doc! { "rating.count" => (-1) });
            let cursor = mongo_utils::create_artist_cursor(&collection, Some(query), Some(option))
                .ok().expect("Failed to execute find.");
            for (i, artist) in cursor.filter_map(|item| item).enumerate() {
                println!("{:?}", artist);
                if i == 9 { break; }
            }
        },
        None => println!("pass tag"),
    }
}