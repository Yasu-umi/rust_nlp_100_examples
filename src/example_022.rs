#!rust run

extern crate regex;
use regex::Regex;
mod lib;

fn main() {
    let re = Regex::new(r"^\[\[Category:(.*?)(\|.*)*\]\]$").unwrap();
    let texts = lib::fetch::country_texts("イギリス");
    let lines = texts.iter()
        .flat_map(|t| t.lines())
        .filter_map(|l| re.captures(l))
        .map(|cap| cap[1].to_string())
        .collect::<Vec<String>>();
    for line in lines {
        println!("{:?}", line);
    }
}
