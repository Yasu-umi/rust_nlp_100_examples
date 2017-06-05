#!rust run

extern crate gnuplot;
extern crate nlp_100_examples;

use std::env;

use gnuplot::{Figure, AxesCommon, LineWidth, BorderColor};

use nlp_100_examples::*;


fn draw_histograph(filepath: &str) {
    let config = config::Config::new().expect("Failed to load config");
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let vec = mecab_utils::get_words_sorted_by_freq(text);

        let x = 1..(vec.iter().len());
        let y = vec.iter().map(|&(_, n)| n);

        let mut fg = Figure::new();
        fg.axes2d()
            .set_x_label("Log-Rank", &[])
            .set_y_label("Log-Freq", &[])
            .set_x_log(Some(10f64))
            .set_y_log(Some(10f64))
            .points(x, y, &[LineWidth(1.0), BorderColor("black")]);
        fg.set_terminal("encoding", "utf8");
        fg.set_terminal("png", filepath);
        fg.show();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filepath) => draw_histograph(filepath),
        None => println!("{}", "specify output file path"),
    }
}
