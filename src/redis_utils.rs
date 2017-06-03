extern crate redis;

use self::redis::{FromRedisValue, Commands};
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

pub fn get_names_iter<T: FromRedisValue>(connect: &redis::Connection) -> Result<redis::Iter<T>, redis::RedisError> {
    connect.scan()
}

pub fn get_areas_iter<'a, T: FromRedisValue>(connect: &'a redis::Connection) -> Result<impl Iterator<Item=String> + 'a, redis::RedisError>  {
    let iter = try!(get_names_iter::<String>(connect));
    Ok(iter.flat_map(move |name| get_area_by_name(connect, name.as_str())))
}