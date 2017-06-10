#!rust run#!rust run

extern crate nlp_100_examples;

use nlp_100_examples::*;


fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");
    let client = fetch::create_client();
    let all_files = fetch::tar_gz_files(client, config.movie_review_data_url)
        .expect("Failed to fetch tar.gz");
    let raw_texts = all_files.into_iter()
        .filter(|&(_, ref path)| path.find("pos").is_some() || path.find("neg").is_some())
        .map(|(raw_text, _)| raw_text);
    let lines = sentiment_utils::create_lines_from_latin1(raw_texts);

    let stop_words = sentiment_utils::sorted_by_frequent_terms_from_lines(&lines)
        .take(100).collect::<Vec<String>>();

    if let Some(wn) = wordnet_utils::create_wordnet_stemmter() {
        for feature in sentiment_utils::get_features_from_line(&wn, lines.iter(), stop_words) {
            println!("{:?}", feature);
        }
    }
}