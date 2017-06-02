extern crate redis;

use redis::redis::Commands;
use artist::Artist;

pub fn create_connect(url: &str) -> Result<redis::Connection, redis::RedisError> {
  let client = try!(redis::Client::open(url));
  let connect = try!(client.get_connection());
  Ok(connect)
}

pub fn set_artists(connect: &redis::Connection, artists: Vec<Artist>) -> Result<(), redis::RedisError> {
    for artist in artists {
        if let Some(area) = artist.area {
            let _: () = try!(connect.set(artist.name, area));
        }
    }
    Ok(())
}