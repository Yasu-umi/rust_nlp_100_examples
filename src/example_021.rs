#!rust run

extern crate regex;
use regex::Regex;
mod lib;

fn main() {
    let re = Regex::new(".*Category.*").unwrap();
    let texts = lib::fetch::country_texts("イギリス");
    let lines =
        texts.iter().flat_map(|t| t.lines()).filter(|l| re.is_match(l)).collect::<Vec<&str>>();
    for line in lines {
        println!("{:?}", line);
    }
}
