extern crate redis;

use redis::redis::Commands;
use fetch;

pub fn create_connect(url: &str) -> Result<redis::Connection, redis::RedisError> {
  let client = try!(redis::Client::open(url));
  let connect = try!(client.get_connection());
  Ok(connect)
}

pub fn set_artists(url: &str) -> Result<(), redis::RedisError> {
    let connect = try!(create_connect(url));
    let artists = fetch::gz_artists_by_line("http://www.cl.ecei.tohoku.ac.jp/nlp100/data/artist.json.gz");
    for artist in artists {
        if let Some(area) = artist.area {
            let _: () = try!(connect.set(artist.name, area));
        }
    }
    Ok(())
}