#!rust run

extern crate regex;
extern crate nlp_100_examples;

use regex::Regex;

use nlp_100_examples::*;


fn main() {
    let re = Regex::new(r"(File|ファイル):(.*\.[a-zA-Z0-9]+)\|.*").unwrap();
    let texts = fetch::country_texts("イギリス");
    let lines = texts.iter()
        .flat_map(|t| t.lines())
        .filter_map(|l| re.captures(l))
        .map(|cap| cap[2].to_string())
        .collect::<Vec<String>>();
    for line in lines {
        println!("{:?}", line);
    }
}
