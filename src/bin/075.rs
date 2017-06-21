#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;

use std::collections::HashSet;
use std::collections::hash_map::RandomState;
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

    let pos_lines_len = pos_lines.len();
    let ng_lines_len = neg_lines.len();

    let answers = sentiment_utils::create_answers(pos_lines.iter(), neg_lines.iter());

    let lines = pos_lines.iter().map(|line| line.clone())
        .chain(neg_lines.iter().map(|line| line.clone()))
        .collect::<Vec<_>>();

    let stop_words = sentiment_utils::get_stop_words(lines.iter())
        .collect::<HashSet<String, RandomState>>();
    println!("stop words len {}", stop_words.len());

    let wn = wordnet_utils::create_wordnet_stemmter()
        .expect("Failed to create wordnet stemmer");

    let features = sentiment_utils::get_features_from_lines(&wn, lines.iter(), &stop_words)
        .collect::<Vec<_>>();
    let all_features = sentiment_utils::create_all_features(&features, &config.others_token);
    let feature_len = all_features.len();
    println!("features_vec len {}", feature_len);

    let features_vec = sentiment_utils::create_features_vec(
        features.iter(),
        &all_features,
        pos_lines_len + ng_lines_len,
        feature_len
    );

    let answers_len = answers.len();
    println!("answers len {}", answers_len);

    let mut lr = logistic_regression::LogisticRegressionBuilder::new()
        .feature_len(feature_len)
        .build();
    lr.learn(&features_vec, &answers, 1000);

    let weights = lr.get_weights();
    let mut weights_features = weights.iter()
        .zip(all_features.iter())
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
