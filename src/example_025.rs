#!rust run

mod lib;


fn main() {
    fn formatter(t: String) -> String {
        t
    };
    let hash = lib::fetch::get_template_hash("イギリス", formatter);
    for (key, value) in &hash {
        println!("{}: {}", key, value);
    }
}
