extern crate hyper;
extern crate flate2;
extern crate serde_json;
extern crate regex;

use std::io;
use std::ops::Index;
use std::error::Error;
use std::collections::HashMap;

use self::hyper::{client, header};
use self::flate2::read::GzDecoder;
use self::serde_json::Value;
use self::regex::Regex;

use artist::Artist;

pub fn create_client() -> client::Client {
    client::Client::new()
}

pub fn reader(client: client::Client, url: &str) -> Result<impl io::BufRead, Box<Error>> {
    let res = try!(client.get(url).send());
    Ok(io::BufReader::new(res))
}

pub fn string(client: client::Client, url: &str) -> Result<String, Box<Error>> {
    use std::io::Read;
    let mut reader = try!(reader(client, url));
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    Ok(text)
}

pub fn gz_reader(client: client::Client, url: &str) -> Result<impl io::BufRead, Box<Error>> {
    let mut req = client.get(url);
    let mut headers = header::Headers::new();
    headers.set(header::AcceptEncoding(vec![header::qitem(header::Encoding::Gzip)]));
    req = req.headers(headers);

    let res = try!(req.send());
    let decorder = try!(GzDecoder::new(res));
    Ok(io::BufReader::new(decorder))
}

pub fn gz_string_by_line<'a>(url: &'a str) -> Result<impl Iterator<Item=String> + 'a, Box<Error>> {
    use self::io::BufRead;
    let reader = try!(gz_reader(create_client(), url));
    Ok(reader.lines().filter_map(|res_line) if let Ok(line) = res_line { Some(line) } else { None })
}

pub fn gz_json_by_line<'a>(url: &'a str) -> Result<impl Iterator<Item=Value> + 'a, Box<Error>> {
    let lines = try!(gz_string_by_line(url));
    Ok(lines.flat_map(|line| serde_json::from_str(line.as_str())))
}

pub fn gz_artists_by_line<'a>(url: &'a str) -> Result<impl Iterator<Item=Artist> + 'a, Box<Error>> {
    let lines = try!(gz_string_by_line(url));
    Ok(lines.flat_map(|line| serde_json::from_str(line.as_str())))
}

pub fn country_texts<'a>(country_re: &str) -> Result<impl Iterator<Item=String> + 'a, Box<Error>> {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz";
    let jsons = try!(gz_json_by_line(url));
    let re = try!(Regex::new(country_re));
    Ok(jsons.filter_map(move |json| {
        if let (&Value::String(ref title), &Value::String(ref text)) = (json.index("title"), json.index("text")) {
            if re.is_match(title) {
                Some(text.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }))
}

pub fn get_template_hash<F: Fn(String) -> String>(country: &str, formatter: F)
    -> Result<HashMap<String, String>, Box<Error>> {
    let spliter = try!(Regex::new(r"\n[\|}]"));
    let re = try!(Regex::new(r"(?s)^(.*?)\s=\s(.*)(?-s)$"));
    let mut hash = HashMap::new();

    let sets = try!(country_texts(country))
        .flat_map(|l|
            spliter.split(&*l)
                .filter_map(|t| re.captures(t))
                .map(|cap| (cap[1].to_owned(), formatter(cap[2].to_owned())))
                .collect::<Vec<(String, String)>>()
        );
    for set in sets {
        hash.insert(set.0, set.1);
    }
    Ok(hash)
}
