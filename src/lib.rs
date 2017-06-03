#![feature(conservative_impl_trait)]

#[macro_use]
extern crate serde_derive;

pub mod n_gram;
pub mod fetch;
pub mod structs;
pub mod mecab_utils;
pub mod artist;
pub mod redis_utils;