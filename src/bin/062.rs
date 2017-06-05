#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;
use std::env;

fn main() {
    let config = config::Config::new().expect("Failed to load config");
    if let Some(area) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(connect) = redis_utils::create_connect(config.redis_host.as_str()) {
            if let Ok(iter) = redis_utils::get_values_iter::<String, String>(&connect) {
                let count = iter.fold(0, |sum, _area| {
                    if &_area == area { sum + 1 } else { sum }
                });
                println!("{}", count);
            }
        }
    } else {
        println!("pass area");
    }
}
