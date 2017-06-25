
extern crate serde;
extern crate bincode;

use self::bincode::{serialize_into, Infinite};

use std::fs;
use std::io::{Read, BufReader, BufWriter};

pub fn dump<T>(path: &String, data: &T)
    where T: serde::ser::Serialize {
    let f = fs::File::create(path)
        .expect(format!("Failed to create {}", path).as_str());
    let mut w = BufWriter::new(&f);
    serialize_into(&mut w, &data, Infinite)
        .expect(format!("Failed to dump {}", path).as_str());
    println!("dump {}", path);
}

pub fn read_bin(path: &String) -> Vec<u8> {
    let f_bin = fs::File::open(path)
        .expect(format!("Failed to open {}", path).as_str());
    let mut file_bin = BufReader::new(&f_bin);
    let mut bin = Vec::new();
    file_bin.read_to_end(&mut bin)
        .expect(format!("Failed to read {}", path).as_str());
    bin
}