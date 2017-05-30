#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

fn tuple_combinator<'a, T>(vec: &'a Vec<T>) -> Vec<(&'a T, &'a T)> {
  let mut tuple_vec: Vec<(&'a T, &'a T)> = Vec::new();
  let len = vec.len();
  for i in 0..len {
    if i == len - 1 { continue; }
    for j in (i + 1)..len {
      if let (Some(i_item), Some(j_item)) = (vec.get(i), vec.get(j)) {
        tuple_vec.push((i_item, j_item));
      }
    }
  }
  tuple_vec
}

fn join_by_arrow(acc: String, item: String) -> String {
    if acc.len() == 0 { item } else { acc + " -> " + item.as_str() }
}

fn main() {
    let text = fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");

    let chunked_sentences = structs::Chunk::from_sentences(text);

    for chunked_sentence in chunked_sentences {
        let has_noun_chunks: Vec<&structs::Chunk> = chunked_sentence.iter().filter(|chunk| chunk.has_noun()).collect();
        if has_noun_chunks.len() == 0 { continue; }
        let has_noun_chunk_tuples = tuple_combinator(&has_noun_chunks);
        for (has_noun_chunk_a, has_noun_chunk_b) in has_noun_chunk_tuples {
          let chunk_a = has_noun_chunk_a.replace_noun("X");
          let chunk_b = has_noun_chunk_b.replace_noun("Y");
          let a_to_root = chunk_a.to_root(chunked_sentence.iter().collect());
          let b_to_root = chunk_b.to_root(chunked_sentence.iter().collect());
          if a_to_root.contains(&&chunk_b) {
              let mut surfaces: Vec<String> = Vec::new();
              for chunk in a_to_root {
                if chunk == &chunk_b {
                  if let Some(morph) = chunk_b.morphs.iter().find(|morph| morph.pos == "名詞") {
                    surfaces.push(morph.surface.clone());
                  }
                  break;
                } else {
                  surfaces.push(chunk.surfaces());
                }
              }
              let path = surfaces.into_iter().fold(String::new(), join_by_arrow);
              println!("{}", path);
          } else {
              if let Some(chunk_c) = a_to_root.iter().find(|chunk| b_to_root.contains(chunk)) {
                let mut a_surfaces: Vec<String> = Vec::new();
                for chunk in a_to_root.iter() {
                  if chunk == chunk_c { break; } else { a_surfaces.push(chunk.surfaces()); }
                }
                let a_path = a_surfaces.into_iter().fold(String::new(), join_by_arrow);

                let mut b_surfaces: Vec<String> = Vec::new();
                for chunk in b_to_root.iter() {
                  if chunk == chunk_c { break; } else { b_surfaces.push(chunk.surfaces()); }
                }
                let b_path = b_surfaces.into_iter().fold(String::new(), join_by_arrow);

                println!("{}|{}|{}", a_path, b_path, chunk_c.surfaces());
              }
          }
        }
    }
}
