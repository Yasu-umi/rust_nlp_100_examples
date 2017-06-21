#!rust run

extern crate nlp_100_examples;
extern crate ndarray;

use nlp_100_examples::*;
use ndarray::{Array1, Array2};

use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;


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

    let pos_lines = sentiment_utils::create_lines_from_latin1(pos_raw_texts).collect::<Vec<String>>();
    let neg_lines = sentiment_utils::create_lines_from_latin1(neg_raw_texts).collect::<Vec<String>>();

    let pos_lines_len = pos_lines.len();
    let ng_lines_len = neg_lines.len();

    let answers_iter = pos_lines.iter().map(|_| 1f32).chain(neg_lines.iter().map(|_| 0f32));
    let answers = Array1::<f32>::from_iter(answers_iter);

    let lines = pos_lines.iter().map(|line| line.clone())
        .chain(neg_lines.iter().map(|line| line.clone()))
        .collect::<Vec<String>>();

    let stop_words = sentiment_utils::get_stop_words(lines.iter())
        .collect::<HashSet<String, RandomState>>();
    println!("stop words len {}", stop_words.len());

    if let Some(wn) = wordnet_utils::create_wordnet_stemmter() {
        println!("get wordnet stemmer");

        let features: Vec<Vec<(String, _)>> = sentiment_utils::get_features_from_lines(&wn, lines.iter(), &stop_words).collect();
        let org_all_features = features.iter().flat_map(|vec| vec.iter().map(|&(ref feature, _)| feature));
        let all_features_set: HashSet<&String, RandomState> = HashSet::from_iter(org_all_features);
        let all_features = Vec::from_iter(all_features_set.into_iter());

        let feature_len = all_features.len() + 1;
        println!("feature_vec len {}", feature_len);

        let mut feature_vec = Array2::<f32>::zeros((pos_lines_len + ng_lines_len, feature_len));
        for (i, _vec) in features.iter().enumerate() {
            feature_vec[[i, 0]] = 1f32;
            let set = _vec.into_iter()
                .map(|&(ref feature, _)| feature)
                .collect::<HashSet<&String, RandomState>>();
            for (j, term) in all_features.iter().enumerate() {
                if set.contains(term) {
                    feature_vec[[i, j+1]] = 1f32;
                }
            }
        }

        let answers_len = answers.len();
        println!("answers len {}", answers_len);

        let mut lr = logistic_regression::LogisticRegressionBuilder::new()
            .feature_len(feature_len)
            .build();
        lr.learn(&feature_vec, &answers, 1000);

        let predict = lr.predict(&feature_vec);
        let mut correct = 0;
        for (predict, answer) in predict.iter().zip(answers.iter()) {
            let predict_answer = if *predict > 0.5f32 { 1f32 } else { 0f32 };
            let proba =  2f32 * if *predict > 0.5f32 { *predict - 0.5f32 } else { 0.5f32 - *predict };
            println!("{}\t{}\t{}", answer, predict_answer, proba);
            if &predict_answer == answer { correct += 1; }
        }
    }
}
