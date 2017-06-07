#!rust run

extern crate nlp_100_examples;
extern crate bson;

use nlp_100_examples::*;
use bson::ordered::OrderedDocument;

fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");

    let collection = mongo_utils::create_collection(&config)
        .expect("Failed to initialize mongo client");

    mongo_utils::drop_and_create_index(&collection);

    let artists = fetch::gz_artists_by_line(config.artists_json_url.as_str())
        .ok().expect("Failed to fetch artists");
    let artists_vec = artists.collect::<Vec<_>>();
    let artists_len = artists_vec.len();

    mongo_utils::insert_artists(&collection, artists_vec)
        .expect("Failed to insert");

    let count = collection.count(Some(OrderedDocument::new()), None)
        .ok().expect("Failed to execute count.");
    println!("insert {} / {}", count, artists_len);
}