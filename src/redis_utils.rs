extern crate redis;
extern crate serde_json;

use self::redis::{ToRedisArgs, FromRedisValue, Commands, cmd};
use artist::Artist;

pub fn create_connect(url: &str) -> Result<redis::Connection, redis::RedisError> {
  let client = try!(redis::Client::open(url));
  let connect = try!(client.get_connection());
  Ok(connect)
}

pub fn flush_and_set_multiple<'a, T, U>(connect: &redis::Connection, items: T)
    -> Result<(), redis::RedisError>
    where T: Iterator<Item=(U, U)>, U: ToRedisArgs {
    cmd("FLUSHDB").execute(connect);
    connect.set_multiple(items.collect::<Vec<(_, _)>>().as_slice())
}

pub fn set_name_area<T: Iterator<Item=Artist>>(connect: &redis::Connection, artists: T)
    -> Result<(), redis::RedisError> {
    let formatter = |artist: Artist| {
        let name = artist.name;
        artist.area.map(|area| (name, area))
    };
    set_from_iter(connect, artists, formatter)
}

pub fn set_name_tags<T: Iterator<Item=Artist>>(connect: &redis::Connection, artists: T)
    -> Result<(), redis::RedisError> {
    let formatter = |artist: Artist| {
        let name = artist.name;
        artist.tags.and_then(|tags| {
            if let Ok(tags_json) = serde_json::to_string(&tags) {
                Some((name, tags_json))
            } else {
                None
            }
        })
    };
    set_from_iter(connect, artists, formatter)
}

pub fn set_from_iter<I, T, U, F>(connect: &redis::Connection, items: T, formatter: F)
    -> Result<(), redis::RedisError>
    where T: Iterator<Item=I>, U: ToRedisArgs, F: Fn(I) -> Option<(U, U)> {
    flush_and_set_multiple(connect, items.into_iter().filter_map(formatter))
}

pub fn get_value_by_key<K, V>(connect: &redis::Connection, key: K)
    -> Result<V, redis::RedisError>
    where K: ToRedisArgs, V: FromRedisValue {
    let res: V = try!(connect.get(key));
    Ok(res)
}

pub fn get_keys_iter<V: FromRedisValue>(connect: &redis::Connection)
    -> Result<redis::Iter<V>, redis::RedisError> {
    connect.scan()
}

pub fn get_values_iter<'a, K, V>(connect: &'a redis::Connection)
    -> Result<impl Iterator<Item=V> + 'a, redis::RedisError>
    where K: ToRedisArgs + FromRedisValue + 'a, V : FromRedisValue + 'a {
    let iter = try!(get_keys_iter::<K>(connect));
    Ok(iter.flat_map(move |key| get_value_by_key::<K, V>(connect, key)))
}
