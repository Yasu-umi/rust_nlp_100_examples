#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;
use std::env;

fn main() {
    let config = config::Config::new().unwrap();
    if let Some(name) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(connect) = redis_utils::create_connect(config.redis_host.as_str()) {
            if let Ok(area) = redis_utils::get_value_by_key::<&String, String>(&connect, name) {
                println!("{}", area);
            } else {
                println!("not found");
            }
        }
    } else {
        println!("pass name");
    }
}
