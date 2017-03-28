#!rust run

use std::collections::HashMap;

mod lib;


fn main() {
  let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
  let mappings = lib::mecab_utils::feature_mappings(input);

  let mut counter: HashMap<String, u32> = HashMap::new();
  for mapping in mappings {
    let value = counter.entry(mapping.surface).or_insert(0);
    *value += 1;
  }
  let mut sorted_counter_tumple = counter.iter().collect::<Vec<(&String, &u32)>>();
  sorted_counter_tumple.sort_by(|&(_, value_a), &(_, value_b)| value_b.cmp(value_a));

  for (key, value) in sorted_counter_tumple {
    println!("{}: {}", key, value)
  }
}
