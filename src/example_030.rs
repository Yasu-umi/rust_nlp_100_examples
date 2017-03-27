#!rust run

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);
  println!("{}", mappings.len());
  for mapping in mappings {
    println!("{}", mapping);
  }
}
