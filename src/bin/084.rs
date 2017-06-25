#!rust run

extern crate nlp_100_examples;
extern crate bincode;

use nlp_100_examples::*;
use bincode::deserialize;

use std::env;
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;


fn main() {
    let args: Vec<String> = env::args().collect();
    let n = u32::from_str(args.get(1).expect("pass N"))
        .expect("Failed to parse N") as f64;

    let config = config::Config::new()
        .expect("Failed to load config");

    if !Path::new(&config.tc_counter_bin_path).exists()
        || !Path::new(&config.ti_counter_bin_path).exists()
        || !Path::new(&config.ci_counter_bin_path).exists() {
       println!("Please cargo run --bin 083");
       return
    }

    let tc_counter_bin = bin_utils::read_bin(&config.tc_counter_bin_path);
    let tc_counter: HashMap<(usize, usize), usize> = deserialize(&tc_counter_bin)
        .expect("Failed to deserialize tc_counter");

    let ti_counter_bin = bin_utils::read_bin(&config.ti_counter_bin_path);
    let ti_counter: HashMap<usize, (usize, usize)> = deserialize(&ti_counter_bin)
        .expect("Failed to deserialize ti_counter");

    let ci_counter_bin = bin_utils::read_bin(&config.ci_counter_bin_path);
    let ci_counter: HashMap<usize, (usize, usize)> = deserialize(&ci_counter_bin)
        .expect("Failed to deserialize ci_counter");

    println!("read bins");

    let mut matrix = dok_matrix::DokMatrix::new(ti_counter.len(), ci_counter.len(), 0f64);

    for (&(t, c), f_tc) in tc_counter.iter() {
        if f_tc >= &10 {
            let (f_t, t_i) = *ti_counter.get(&t)
                .expect(format!("can't get {} from ti_counter", t).as_str());
            let (f_c, c_i) = *ci_counter.get(&c)
                .expect(format!("can't get {} from ci_counter", c).as_str());
            let ppmi = ((n * &(*f_tc as f64)) / (&(f_t as f64) * &(f_c as f64))).ln().max(0f64);
            matrix.set(t_i, c_i, ppmi).unwrap();
        }
    }

    println!("maatrix len {}", matrix.len());
    bin_utils::dump(&config.matrix_bin_path, &matrix);
}
