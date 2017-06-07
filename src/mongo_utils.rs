extern crate mongodb;

use ::bson::{Bson, to_bson};
use self::mongodb::{Client, ThreadedClient, coll};
use self::mongodb::db::ThreadedDatabase;

use config::Config;
use artist::Artist;

pub fn create_collection(config: &Config) -> Option<coll::Collection> {
    Client::connect(config.mongo_host.as_str(), 27017)
        .map(|client| client.db("test").collection("artist")).ok()
}

pub fn create_sparse_option() -> Option<coll::options::IndexOptions> {
    let mut option = coll::options::IndexOptions::new();
    option.sparse = Some(true);
    Some(option)
}

pub fn drop_and_create_index(collection: &coll::Collection) -> Option<Vec<String>> {
    let index_docs = vec![
        (doc!{"name" => 1}, None),
        (doc!{"aliases.name" => 1}, create_sparse_option()),
        (doc!{"tags.value" => 1}, create_sparse_option()),
        (doc!{"rating.value" => 1}, create_sparse_option())
    ];
    collection.drop()
        .ok().map(|_| {
            index_docs.into_iter().filter_map(|(index_doc, option)|
                collection.create_index(index_doc, option).ok()
            ).collect()
        })
}

pub fn insert_artists(collection: &coll::Collection, artists: Vec<Artist>) -> Option<()> {
    let artists_len = artists.len();
    let mut buffer_vec = Vec::new();
    for (i, artist) in artists.iter().enumerate() {
        if let Ok(Bson::Document(document)) = to_bson(&artist) {
            buffer_vec.push(document);
        }
        if i % 1000 == 999 || i == artists_len - 1 {
            if let Ok(_) = collection.insert_many(buffer_vec, None) {
                buffer_vec = Vec::new();
            } else {
                return None
            }
            
        }
    }
    Some(())
}
