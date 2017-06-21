#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

use std::cmp::Ordering::Equal;


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");
    let client = fetch::create_client();
    let all_files = fetch::tar_gz_files(client, config.movie_review_data_url)
        .expect("Failed to fetch tar.gz");

    let pos_raw_texts = all_files.iter()
        .filter(|&&(_, ref path)| path.find("pos").is_some())
        .map(|&(ref raw_text, _)| raw_text);
    let neg_raw_texts = all_files.iter()
        .filter(|&&(_, ref path)| path.find("neg").is_some())
        .map(|&(ref raw_text, _)| raw_text);

    let pos_lines = sentiment_utils::create_lines_from_latin1(pos_raw_texts)
        .collect::<Vec<_>>();
    let neg_lines = sentiment_utils::create_lines_from_latin1(neg_raw_texts)
        .collect::<Vec<_>>();

    let learning_data = sentiment_utils::create_answers_iter(pos_lines.iter(), neg_lines.iter())
        .zip(pos_lines.iter().chain(neg_lines.iter()))
        .collect::<Vec<_>>();

    let wn = wordnet_utils::create_wordnet_stemmter()
        .expect("Failed to create wordnet stemmer");

    let learning_n = 1000;

    let learning_result = sentiment_utils::learning(&wn, learning_data, learning_n, &config.others_token);

    let weights = learning_result.lr.get_weights();
    let mut weights_features = weights.iter()
        .zip(learning_result.all_features.iter())
        .collect::<Vec<_>>();
    weights_features.sort_by(|a, b| b.0.partial_cmp(a.0).unwrap_or(Equal));
    println!("top 10");
    for &(weight, feature) in weights_features.iter().take(10) {
        println!("{}\t{}", weight, feature);
    }
    weights_features.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap_or(Equal));
    println!("worst 10");
    for &(weight, feature) in weights_features.iter().take(10) {
        println!("{}\t{}", weight, feature);
    }
}
