#!rust run

extern crate nlp_100_examples;
extern crate iron;
extern crate router;
extern crate serde_json;

#[macro_use(doc, bson)]
extern crate bson;

use nlp_100_examples::*;
use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use router::Router;

use std::error::Error;

fn artists_by_name(config: &config::Config, name: &str)
    -> Result<Vec<artist::Artist>, Box<Error>> {
    let collection = try!(mongo_utils::create_collection(config).ok_or("Failed to create collection"));
    let query = doc! { "name" => name };
    let cursor = try!(mongo_utils::create_artist_cursor(&collection, Some(query), None));
    Ok(cursor.filter_map(|item| item).collect())
}
 
fn main() {
    let config = config::Config::new()
        .expect("Failed to load config");
    let iron_host = config.iron_host.clone();

    let mut router = Router::new();
    router.get("/", |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    }, "hello_world");
    router.get("/artist/:name", move |req: &mut Request| {
        if let Some(name) = req.extensions.get::<Router>().map(|query| query.find("name")).unwrap_or(None) {
            if let Ok(artists) = artists_by_name(&config, name) {
                if let Ok(body) = serde_json::to_string(&artists) {
                    return Ok(Response::with((ContentType::json().0, status::Ok, body)));
                }
            }
        }
        Ok(Response::with(status::InternalServerError))
    }, "artist_name");

    Iron::new(router).http(iron_host.as_str())
        .expect("Failed to create server");
}