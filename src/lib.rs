#![feature(conservative_impl_trait)]

#[macro_use(Deserialize, Serialize)]
extern crate serde_derive;
#[macro_use(doc, bson)]
extern crate bson;

pub mod n_gram;
pub mod fetch;
pub mod structs;
pub mod mecab_utils;
pub mod artist;
pub mod redis_utils;
pub mod config;
pub mod mongo_utils;
pub mod sentiment_utils;
pub mod wordnet_utils;
pub mod logistic_regression;
pub mod bin_utils;
pub mod dok_matrix;
