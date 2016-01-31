#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate urlshortener_core as core;

use mongodb::{Client, ThreadedClient};

fn get_mongo_client() -> mongodb::Client {
    let client = Client::connect("localhost", 27017)
        .ok().expect("Failed to initialize standalone client.");

    client
}

pub mod repository;