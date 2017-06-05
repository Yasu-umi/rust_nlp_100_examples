#!rust run

extern crate gnuplot;
extern crate nlp_100_examples;

use std::env;
use std::iter::repeat;

use gnuplot::{Figure, AxesCommon, Color, LineWidth, BorderColor, Major, Fix, Font};

use nlp_100_examples::*;


fn draw_top10_freq_bar_graph(filepath: &str) {
    let config = config::Config::new().expect("Failed to load config");
    if let Ok(text) = fetch::string(fetch::create_client(), config.neko_text_url.as_str()) {
        let mut sorted_tumple_vec = mecab_utils::get_words_sorted_by_freq(text);
        sorted_tumple_vec.split_off(10);

        let x = 0..sorted_tumple_vec.len();
        let y = sorted_tumple_vec.iter().map(|&(_, v)| v);
        let w = repeat(0.5f32);
        let labels = sorted_tumple_vec.iter()
            .enumerate()
            .map(|(idx, &(ref word, _))| Major(idx, Fix(word.clone())));

        let mut fg = Figure::new();
        fg.axes2d()
            .set_x_label("Word", &[])
            .set_y_label("Num", &[])
            .boxes_set_width(x,
                            y,
                            w,
                            &[LineWidth(1.0), Color("gray"), BorderColor("black")])
            .set_x_ticks_custom(labels, &[], &[Font("ipag.ttf", 12f64)]);
        fg.set_terminal("encoding", "utf8");
        fg.set_terminal("png", filepath);
        fg.show();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filepath) => draw_top10_freq_bar_graph(filepath),
        None => println!("{}", "specify output file path"),
    }
}
