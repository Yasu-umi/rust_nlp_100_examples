#!rust run

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);
  println!("{}", mappings.len());
  let mut surfaces: Vec<String> = Vec::new();
  for mapping in mappings {
    if mapping.pos == "動詞" {
      surfaces.push(mapping.surface);
    }
  }
  for surface in surfaces {
    println!("{}", surface)
  }
}
