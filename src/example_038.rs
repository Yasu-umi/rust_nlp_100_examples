#!rust run

extern crate gnuplot;

use std::env;

use gnuplot::{Figure, AxesCommon, Color, LineWidth, BorderColor, Major, Fix};
use std::iter::repeat;

mod lib;


fn draw_histograph(filepath: &str) {
    let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let vec = lib::mecab_utils::get_freq_words_vec(input);
    let kinds_freq_vec = vec.iter()
        .map(|&(n, ref words)| (n, words.len()))
        .collect::<Vec<(u32, usize)>>();

    let x = 0..(kinds_freq_vec.iter().len() - 1);
    let y = kinds_freq_vec.iter().map(|&(_, len)| len);
    let w = repeat(1.0f32);
    let labels = (1..(kinds_freq_vec.iter().len()))
        .filter(|x| (x % 50) == 0)
        .map(|x| Major(x, Fix((x).to_string())));

    let mut fg = Figure::new();
    fg.axes2d()
        .set_x_label("Num", &[])
        .set_y_label("Kinds", &[])
        .boxes_set_width(x, y, w, &[LineWidth(1.0), Color("gray"), BorderColor("black")])
        .set_x_ticks_custom(labels, &[], &[]);
    fg.set_terminal("encoding", "utf8");
    fg.set_terminal("png", filepath);
    fg.show();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filepath) => draw_histograph(filepath),
        None => println!("{}", "specify output file path"),
    }
}
