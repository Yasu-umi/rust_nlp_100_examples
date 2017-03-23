#!rust run

mod lib;

fn main() {
    let texts = lib::fetch::country_texts("イギリス");
    for text in texts {
        println!("{:?}", text);
    }
}
