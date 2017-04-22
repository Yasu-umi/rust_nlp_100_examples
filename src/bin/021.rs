#!rust run

extern crate regex;
extern crate nlp_100_examples;

use regex::Regex;

use nlp_100_examples::*;

fn main() {
    let re = Regex::new(".*Category.*").unwrap();
    let texts = fetch::country_texts("イギリス");
    let lines =
        texts.iter().flat_map(|t| t.lines()).filter(|l| re.is_match(l)).collect::<Vec<&str>>();
    for line in lines {
        println!("{:?}", line);
    }
}
