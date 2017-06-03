#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new().unwrap();
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let chunked_sentences = structs::Chunk::from_sentences(text);

        for chunked_sentence in chunked_sentences {
            for i in 0..chunked_sentence.len() {
                if i == 0 { continue; }
                let first_idx = i - 1;
                if let (Some(first), Some(second)) = (chunked_sentence.get(first_idx), chunked_sentence.get(i)) {
                    let morphs_len = first.morphs.len();
                    if morphs_len <= 1 { continue; }
                    if let (Some(first_morph), Some(second_morph)) = (first.morphs.get(morphs_len-2), first.morphs.get(morphs_len-1)) {
                        if first_morph.pos1 == "サ変接続" && second_morph.pos == "助詞" && second_morph.surface == "を" {
                            let base = format!(
                                "{}{}{}",
                                first_morph.surface,
                                second_morph.surface,
                                second.morphs_of_pos("動詞")
                                    .iter()
                                    .map(|morph| morph.base.clone())
                                    .fold(String::new(), |acc, cur| acc + cur.as_str())
                            );

                            let mut chunks: Vec<(String, &str)> = second.srcs.iter()
                                .filter(|&&src| src != first_idx)
                                .filter_map(|src| chunked_sentence.get(*src))
                                .filter_map(|chunk| {
                                    if let Some(morph) = chunk.morphs_of_pos("助詞").last() {
                                        Some((chunk.surfaces(), morph.surface.as_str()))
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            if chunks.len() == 0 { continue; }
                            chunks.sort_by_key(|&(_, key)| key);

                            let particles = chunks.iter().map(|&(_, particle)| particle).fold(String::new(), (|acc, item| acc + " " + item));
                            let surfaces = chunks.into_iter().fold(String::new(), (|acc, (surface, _)| acc + " " + surface.as_str()));
                            println!("{}\t{}\t{}", base, particles, surfaces);
                        }
                    }
                }
            }
        }
    }
}
