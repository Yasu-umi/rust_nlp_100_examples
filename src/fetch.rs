extern crate hyper;
extern crate flate2;
extern crate serde_json;
extern crate regex;

use std::io::Read;
use std::collections::HashMap;

use self::hyper::client::Client;
use self::hyper::header::{Headers, AcceptEncoding, Encoding, qitem};

use self::flate2::read::GzDecoder;

use self::serde_json::Value;

use self::regex::Regex;

use artist::Artist;


#[allow(dead_code)]
pub fn text(url: &str) -> String {
    let client = Client::new();
    let req = client.get(url);
    let mut res = req.send().unwrap();
    let mut buf = String::new();
    let _ = res.read_to_string(&mut buf);
    buf
}

#[allow(dead_code)]
pub fn gz_text(url: &str) -> String {
    let client = Client::new();

    let mut req = client.get(url);
    let mut headers = Headers::new();
    headers.set(AcceptEncoding(vec![qitem(Encoding::Gzip)]));
    req = req.headers(headers);

    let res = req.send().unwrap();
    let mut decoder = GzDecoder::new(res).unwrap();
    let mut buf = String::new();

    let _ = decoder.read_to_string(&mut buf);
    buf
}

#[allow(dead_code)]
pub fn gz_json_by_line(url: &str) -> Vec<Value> { 
    let res = gz_text(url);
    res.as_str().trim().split('\n').flat_map(serde_json::from_str).collect()
}

#[allow(dead_code)]
pub fn gz_artists_by_line(url: &str) -> Vec<Artist> { 
    let res = gz_text(url);
    let lines = res.lines().collect::<Vec<&str>>();
    let mut vec: Vec<Artist> = Vec::with_capacity(lines.len());
    for line in lines {
        if let Ok(artist) = serde_json::from_str(line) {
            vec.push(artist);
        }
    }
    vec
}

#[allow(dead_code)]
pub fn country_texts(country_re: &str) -> Vec<String> {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz";
    let buf = gz_json_by_line(url);
    let mut texts: Vec<String> = Vec::new();
    let re = Regex::new(country_re).unwrap();
    for elm in buf {
        if let Value::String(ref title) = elm["title"] {
            if let Value::String(ref text) = elm["text"] {
                if re.is_match(title) {
                    texts.push(text.to_owned());
                }
            }
        }
    }
    texts
}

#[allow(dead_code)]
pub fn get_template_hash<F: Fn(String) -> String>(country: &str,
                                                  formatter: F)
                                                  -> HashMap<String, String> {
    let mut hash = HashMap::new();
    let re = Regex::new(r"(?s)^(.*?)\s=\s(.*)(?-s)$").unwrap();
    let spliter = Regex::new(r"\n[\|}]").unwrap();
    let texts = country_texts(country);
    let sets = texts.iter()
        .flat_map(|t| spliter.split(t))
        .filter_map(|l| re.captures(l))
        .map(|cap| (cap[1].to_string(), formatter(cap[2].to_string())));
    for set in sets {
        hash.insert(set.0, set.1);
    }
    hash
}
