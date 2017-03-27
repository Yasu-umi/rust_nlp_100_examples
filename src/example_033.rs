#!rust run

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);
  println!("{}", mappings.len());
  let mut nouns: Vec<String> = Vec::new();
  for mapping in mappings {
    if mapping.pos == "名詞" && mapping.pos1 == "サ変接続" {
      nouns.push(mapping.base);
    }
  }
  for noun in nouns {
    println!("{}", noun)
  }
}
