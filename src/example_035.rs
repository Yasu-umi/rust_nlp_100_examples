#!rust run

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);
  println!("{}", mappings.len());
  let mut nouns: Vec<String> = Vec::new();
  let mut noun_mappings: Vec<lib::mecab_utils::MecabFeatureMapping> = Vec::new();
  for mapping in mappings {
    if mapping.pos == "名詞" {
      noun_mappings.push(mapping);
    } else {
      let len = noun_mappings.len();
      if len > 1 {
        let noun = noun_mappings.iter().map(|mapping| mapping.surface.clone()).collect::<Vec<String>>().concat();
        nouns.push(noun);
      }
      if len > 0 {
        noun_mappings = Vec::new();
      }
    }
  }
  for noun in nouns {
    println!("{}", noun)
  }
}
