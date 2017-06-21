#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


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

    let k = 5;
    let learning_n = 1000;

    let statics = sentiment_utils::k_cross_validation(k, learning_n, pos_lines, neg_lines)
        .iter()
        .fold(logistic_regression::LogisticRegressionStatics::new(), |sum, stats| sum.add_statics(stats));
    println!(
        "予測の正解率: {}\n正例に関する適合率: {}\n再現率: {}\nF1スコア: {}",
        statics.correct_rate() / (k as f32),
        statics.precision_rate() / (k as f32),
        statics.recall_rate() / (k as f32),
        statics.f_value() / (k as f32),
    );
}
