#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

fn main() {
    if let Ok(connect) = redis::create_connect("redis://127.0.0.1/") {
        let artists = nlp_100_examples::fetch::gz_artists_by_line("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/artist.json.gz");
        println!("{:?}", redis::set_artists(&connect, artists));
    }
}
