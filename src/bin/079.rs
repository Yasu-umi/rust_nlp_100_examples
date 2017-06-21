#!rust run

extern crate nlp_100_examples;
extern crate gnuplot;

use nlp_100_examples::*;
use gnuplot::{AxesCommon, Figure, Color};

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => println!("{}", "specify output file path"),
        Some(filepath) => {
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

            let n = 100;
            let data = (0..n)
                .map(|i| {
                    let threshold = 1f32 * (i as f32) / (n as f32);
                    (threshold, learning_result.lr.get_statics(&learning_result.features_vec, &learning_result.answers, threshold))
                })
                .collect::<Vec<_>>();

            let mut fg = Figure::new();
            let _ = fg.axes2d()
                .set_x_label("Threshold", &[])
                .set_y_label("Accuracy", &[])
                .lines(
                    data.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
                    data.iter().map(|&(_, ref statics)| statics.precision_rate()),
                    &[Color("blue")],
                )
                .lines(
                    data.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
                    data.iter().map(|&(_, ref statics)| statics.precision_rate()),
                    &[Color("green")],
                )
                .lines(
                    data.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
                    data.iter().map(|&(_, ref statics)| statics.recall_rate()),
                    &[Color("red")],
                )
                .lines(
                    data.iter().map(|&(x, _)| x).collect::<Vec<_>>(),
                    data.iter().map(|&(_, ref statics)| statics.f_value()),
                    &[Color("black")],
                );
            fg.set_terminal("encoding", "utf8");
            fg.set_terminal("png", filepath);
            fg.show();
        },
    }
}
