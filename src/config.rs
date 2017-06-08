extern crate serde_json;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Read;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub redis_host: String,
    pub mongo_host: String,
    pub neko_text_url: String,
    pub artists_json_url: String,
    pub country_json_url: String,
    pub iron_host: String,
    pub movie_review_data_url: String,
    pub sentiment_path: String,
}

impl Config {
    pub fn new() -> Result<Config, Box<Error>> {
        let path = Path::new("./src/config.json");
        let mut s = String::new();
        let mut file = try!(File::open(&path));
        let _ = try!(file.read_to_string(&mut s));
        let config: Config = try!(serde_json::from_str(s.as_str()));
        Ok(config)
    }
}
