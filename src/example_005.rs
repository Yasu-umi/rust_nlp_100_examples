#!rust run

use std::env;
mod lib;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let optional_unit = args.get(1);
    let optional_n = args.get(2);
    let optional_sentence = args.get(3);
    match (optional_sentence, optional_n, optional_unit) {
        (Some(sentence), Some(str_n), Some(unit)) => {
            match (&*unit.to_string(), str_n.parse::<i32>()) {
                ("word", Ok(n)) => {
                    for words in lib::n_gram::by_word(n, sentence) {
                        println!("{}", words.join(" "));
                    }
                }
                ("str", Ok(n)) => {
                    for n_str in lib::n_gram::by_str(n, sentence) {
                        println!("{}", n_str);
                    }
                }
                (_, Ok(_)) => println!("specify 'str' or 'word' to first argument"),
                (_, _) => println!("specify int as n to second argument"),
            }
        }
        (_, _, _) => println!("specify mode('str' or 'word'), n and sentence"),
    }
}
