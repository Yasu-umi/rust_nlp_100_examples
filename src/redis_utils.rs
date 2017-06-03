extern crate redis;

use self::redis::Commands;
use artist::Artist;

pub fn create_connect(url: &str) -> Result<redis::Connection, redis::RedisError> {
  let client = try!(redis::Client::open(url));
  let connect = try!(client.get_connection());
  Ok(connect)
}

pub fn set_artists(connect: &redis::Connection, artists: Vec<Artist>) -> Result<(), redis::RedisError> {
    let items = artists.into_iter()
        .filter_map(|artist| {
            let name = artist.name;
            artist.area.map(|area| (name, area))
        })
        .collect::<Vec<(String, String)>>();
    connect.set_multiple(items.as_slice())
}

pub fn get_area_by_name(connect: &redis::Connection, name: &str) -> Result<String, redis::RedisError> {
    let res: String = try!(connect.get(name));
    Ok(res)
}