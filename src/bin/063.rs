#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;
use std::env;

fn main() {
    let config = config::Config::new().unwrap();
    if let Some(name) = env::args().collect::<Vec<String>>().get(1) {
        if let (Ok(connect), Ok(artists)) =
            (
                redis_utils::create_connect(config.redis_host.as_str()),
                fetch::gz_artists_by_line(config.artists_json_url.as_str())
            ) {
            println!("{:?}", redis_utils::set_name_tags(&connect, artists));
            if let Ok(tags_json) = redis_utils::get_value_by_key::<&String, String>(&connect, name) {
                println!("{}", tags_json);
            }
        }
    } else {
        println!("pass name");
    }
}
