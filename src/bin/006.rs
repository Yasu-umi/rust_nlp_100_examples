#!rust run

extern crate nlp_100_examples;

use std::env;
use nlp_100_examples::*;

pub struct UniDiffProdStringVecs {
    pub uni: Vec<String>,
    pub diff: Vec<String>,
    pub prod: Vec<String>,
}

fn uni_diff_prod_string_vecs(sentence_1: &str, sentence_2: &str) -> UniDiffProdStringVecs {
    let bi_gramed_sentence_1 = n_gram::by_str(2, sentence_1);
    let bi_gramed_sentence_2 = n_gram::by_str(2, sentence_2);
    let mut uni_vec = Vec::new();
    let mut diff_vec = Vec::new();
    let mut prod_vec = Vec::new();
    for bi_str in bi_gramed_sentence_1 {
        uni_vec.push(bi_str.clone());
        if bi_gramed_sentence_2.contains(&bi_str) {
            prod_vec.push(bi_str.clone());
        } else {
            diff_vec.push(bi_str.clone());
        }
    }
    for bi_str in bi_gramed_sentence_2 {
        if !uni_vec.contains(&bi_str) {
            uni_vec.push(bi_str.clone());
        }
    }
    UniDiffProdStringVecs {
        uni: uni_vec,
        diff: diff_vec,
        prod: prod_vec,
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let optional_sentence_1 = args.get(1);
    let optional_sentence_2 = args.get(2);
    match (optional_sentence_1, optional_sentence_2) {
        (Some(sentence_1), Some(sentence_2)) => {
            let vecs = uni_diff_prod_string_vecs(sentence_1, sentence_2);
            println!("union set      : {:?}", vecs.uni);
            println!("difference set : {:?}", vecs.diff);
            println!("product set    : {:?}", vecs.prod);
        }
        (_, _) => println!("specify 2 sentences"),
    }
}
