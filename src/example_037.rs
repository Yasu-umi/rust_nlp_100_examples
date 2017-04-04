#!rust run

extern crate gnuplot;

use std::env;
use std::collections::HashMap;

use gnuplot::{Figure, AxesCommon, Color, LineWidth, BorderColor, Major, Fix, Font};
use std::iter::repeat;

mod lib;


fn get_words_sorted_by_freq(input: String) -> Vec<(String, u32)> {
    let mappings = lib::mecab_utils::feature_mappings(input);
    let mut counter: HashMap<String, u32> = HashMap::new();
    for mapping in mappings {
        let value = counter.entry(mapping.surface).or_insert(0);
        *value += 1;
    }
    let mut sorted_tumple_vec: Vec<(String, u32)> = counter.iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect();
    sorted_tumple_vec.sort_by(|&(_, value_a), &(_, value_b)| value_b.cmp(&value_a));
    sorted_tumple_vec
}

fn draw_top10_freq_bar_graph(filepath: &str) {
    let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let mut sorted_tumple_vec = get_words_sorted_by_freq(input);
    sorted_tumple_vec.split_off(10);

    let x = 0..sorted_tumple_vec.len();
    let y = sorted_tumple_vec.iter().map(|&(_, v)| v);
    let w = repeat(0.5f32);
    let labels = sorted_tumple_vec.iter().enumerate().map(|(idx, &(ref word, _))| Major(idx, Fix(word.clone())));
    let mut fg = Figure::new();
    fg.axes2d()
        .set_x_label("Word", &[])
        .set_y_label("Num", &[])
        .boxes_set_width(x, y, w, &[LineWidth(1.0), Color("gray"), BorderColor("black")])
        .set_x_ticks_custom(labels, &[], &[Font("ipag.ttf", 12f64)]);
    fg.set_terminal("encoding", "utf8");
    fg.set_terminal("png", filepath);
    fg.show();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filepath) => draw_top10_freq_bar_graph(filepath),
        None => println!("{}", "specify output file path"),
    }
}
