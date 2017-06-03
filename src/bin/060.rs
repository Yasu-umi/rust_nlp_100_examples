#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

fn main() {
    if let Ok(config) = config::Config::new() {
        if let (Ok(connect), Ok(artists)) =
            (
                redis_utils::create_connect(config.redis_host.as_str()),
                fetch::gz_artists_by_line(config.artists_json_url.as_str())
            ) {
            println!("{:?}", redis_utils::set_name_area(&connect, artists));
        }
    }
}
