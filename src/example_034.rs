#!rust run

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);
  println!("{}", mappings.len());
  let mut nouns: Vec<String> = Vec::new();
  for idx in 0..(mappings.len() - 3) {
    let mapping_0 = mappings.get(idx).unwrap();
    let mapping_1 = mappings.get(idx + 1).unwrap();
    let mapping_2 = mappings.get(idx + 2).unwrap();
    if mapping_0.pos == "名詞" && mapping_1.surface == "の" && mapping_2.pos == "名詞" {
      let mut surface = mapping_0.surface.clone();
      surface = surface + mapping_1.surface.as_str();
      surface = surface + mapping_2.surface.as_str();
      nouns.push(surface);
    }
  }
  for noun in nouns {
    println!("{}", noun)
  }
}
