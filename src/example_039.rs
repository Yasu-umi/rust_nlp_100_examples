#!rust run

extern crate gnuplot;

use std::env;

use gnuplot::{Figure, AxesCommon, LineWidth, BorderColor};

mod lib;


fn draw_histograph(filepath: &str) {
    let input = lib::fetch::text("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt");
    let vec = lib::mecab_utils::get_words_sorted_by_freq(input);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(filepath) => draw_histograph(filepath),
        None => println!("{}", "specify output file path"),
    }
}
