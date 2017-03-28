#!rust run

extern crate regex;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;
extern crate url;

use regex::Regex;
use std::io::Read;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;
use url::Url;

mod lib;


fn main() {
    let re1 = Regex::new(r"'{2,5}").unwrap();
    let re2 = Regex::new(r"\[{2}([^|\]]+?\|)*(.+?)\]{2}").unwrap();
    let re3 = Regex::new(r"\{{2}.+?\|.+?\|(.+?)\}{2}").unwrap();
    let re4 = Regex::new(r"<.*?>").unwrap();
    let re5 = Regex::new(r"\[.*?\]").unwrap();
    let formatter = |t: String| {
        let mut tmp_t = (*re1.replace_all(t.as_str(), "")).to_string();
        tmp_t = (*re2.replace_all(tmp_t.as_str(), "$2")).to_string();
        tmp_t = (*re3.replace_all(tmp_t.as_str(), "$2")).to_string();
        tmp_t = (*re4.replace_all(tmp_t.as_str(), "")).to_string();
        tmp_t = (*re5.replace_all(tmp_t.as_str(), "")).to_string();
        tmp_t
    };
    let hash = lib::fetch::get_template_hash(r"イギリス", formatter);
    for (key, value) in &hash {
        if Regex::new("国旗画像").unwrap().is_match(key) {
            let titles = "Image:".to_string() + value;
            let options = &[("action", "query"),
                            ("prop", "imageinfo"),
                            ("iiprop", "url"),
                            ("format", "json"),
                            ("formatversion", "2"),
                            ("utf8", ""),
                            ("continue", ""),
                            ("titles", titles.as_str())];
            let url = hyper::Url::parse(Url::parse_with_params("https://ja.wikipedia.org/w/api.\
                                                                php",
                                                               options)
                    .unwrap()
                    .as_str())
                .unwrap();

            let tls = NativeTlsClient::new().unwrap();
            let connector = HttpsConnector::new(tls);
            let client = Client::with_connector(connector);

            let mut res: String = String::new();
            let _ = client.get(url)
                .send()
                .unwrap()
                .read_to_string(&mut res);
            let value: Value = serde_json::from_str(res.as_str()).unwrap();
            println!("{}", value["query"]["pages"][0]["imageinfo"][0]["url"]);
        }
    }
}
