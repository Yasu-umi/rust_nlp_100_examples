extern crate hyper;
extern crate flate2;
extern crate serde_json;
extern crate regex;
extern crate tar;

use std::io;
use std::io::{Read, BufRead, BufReader, ErrorKind};
use std::ops::Index;
use std::error::Error;
use std::collections::HashMap;

use self::hyper::{client, header};
use self::flate2::read::GzDecoder;
use self::serde_json::Value;
use self::regex::Regex;
use self::tar::Archive;

use artist::Artist;

pub fn create_client() -> client::Client {
    client::Client::new()
}

pub fn reader(client: client::Client, url: &str)
    -> Result<impl BufRead, Box<Error>> {
    let res = try!(client.get(url).send());
    Ok(BufReader::new(res))
}

pub fn string(client: client::Client, url: &str)
    -> Result<String, Box<Error>> {
    let mut reader = try!(reader(client, url));
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    Ok(text)
}

pub fn gz_reader(client: client::Client, url: &str)
    -> Result<impl BufRead, Box<Error>> {
    let mut req = client.get(url);
    let mut headers = header::Headers::new();
    headers.set(header::AcceptEncoding(vec![header::qitem(header::Encoding::Gzip)]));
    req = req.headers(headers);

    let res = try!(req.send());
    let decorder = try!(GzDecoder::new(res));
    Ok(BufReader::new(decorder))
}

pub fn tar_gz_files<'a>(client: client::Client, url: String)
    -> Result<Vec<(Vec<u8>, String)>, Box<Error>> {
    let gz_reader = try!(gz_reader(client, url.as_str()));
    let mut archive = Archive::new(gz_reader);
    let entries = try!(archive.entries());
    Ok(entries.filter_map(|entry| {
        entry.and_then(|mut file| {
            let mut buffer = Vec::new();
            let _ = file.read_to_end(&mut buffer)?;
            let path = try!(
                try!(file.header().path())
                    .to_str()
                    .map(|path_str| path_str.to_string())
                    .ok_or(io::Error::new(ErrorKind::Other, "Failed to parse path"))
                );
            Ok((buffer, path))
        }).ok()
    }).collect())
}

pub fn gz_string_by_line<'a>(url: &'a str)
    -> Result<impl Iterator<Item=String> + 'a, Box<Error>> {
    use self::BufRead;
    let reader = try!(gz_reader(create_client(), url));
    Ok(reader.lines().filter_map(|res_line)
        if let Ok(line) = res_line { Some(line) } else { None }
    )
}

pub fn gz_json_by_line<'a>(url: &'a str)
    -> Result<impl Iterator<Item=Value> + 'a, Box<Error>> {
    let lines = try!(gz_string_by_line(url));
    Ok(lines.flat_map(|line| serde_json::from_str(line.as_str())))
}

pub fn gz_artists_by_line<'a>(url: &'a str)
    -> Result<impl Iterator<Item=Artist> + 'a, Box<Error>> {
    let lines = try!(gz_string_by_line(url));
    Ok(lines.flat_map(|line| serde_json::from_str(line.as_str())))
}

pub fn country_texts<'a>(url: &'a str, country_re: &str)
    -> Result<impl Iterator<Item=String> + 'a, Box<Error>> {
    let jsons = try!(gz_json_by_line(url));
    let re = try!(Regex::new(country_re));
    Ok(jsons.filter_map(move |json| {
        if let (&Value::String(ref title), &Value::String(ref text)) =
            (json.index("title"), json.index("text")) {
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

pub fn get_template_hash<F>(url: &str, country: &str, formatter: F)
    -> Result<HashMap<String, String>, Box<Error>>
    where F: Fn(String) -> String {
    let spliter = try!(Regex::new(r"\n[\|}]"));
    let re = try!(Regex::new(r"(?s)^(.*?)\s=\s(.*)(?-s)$"));
    let mut hash = HashMap::new();

    let sets = try!(country_texts(url, country))
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
