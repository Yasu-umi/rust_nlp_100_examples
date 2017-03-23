#!rust run

extern crate regex;
use regex::Regex;
mod lib;

fn main() {
    let re = Regex::new(r"^(=+)\s*(.*?)\s*(=+)$").unwrap();
    let texts = lib::fetch::country_texts("イギリス");
    let lines = texts.iter()
        .flat_map(|t| t.lines())
        .filter_map(|l| re.captures(l))
        .map(|cap| (cap[2].to_string(), cap[1].to_string().len() - 1))
        .collect::<Vec<(String, usize)>>();
    for line in lines {
        println!("{:?}", line);
    }
}
