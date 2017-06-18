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

    let pos_lines = sentiment_utils::create_lines_from_latin1(pos_raw_texts).collect::<Vec<String>>();
    let neg_lines = sentiment_utils::create_lines_from_latin1(neg_raw_texts).collect::<Vec<String>>();

    let lines = pos_lines.iter().map(|line| line.clone())
        .chain(neg_lines.iter().map(|line| line.clone()))
        .collect::<Vec<String>>();
    let stop_words = sentiment_utils::sorted_by_frequent_terms_from_lines(lines.iter())
        .iter()
        .map(|&(ref word, _)| word.to_lowercase())
        .take(100)
        .collect::<Vec<String>>();

    if let Some(wn) = wordnet_utils::create_wordnet_stemmter() {
        let features = sentiment_utils::get_features_from_lines(&wn, lines.iter(), &stop_words);
        for feature in features {
            println!("{:?}", feature);
        }
    }
}