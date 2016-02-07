#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate urlshortener_core as core;

use std::env;
use mongodb::{Client, ThreadedClient};

fn get_mongo_client() -> mongodb::Client {
    let mongo_host_key = "URLSHORTENER_MONGO_HOST"; 
    let mongo_host: String;
    
    match env::var(mongo_host_key) {
         Ok(val) => mongo_host = val,
         Err(_e) => mongo_host = "localhost".to_string()
    }
    
    let mongo_port_key = "URLSHORTENER_MONGO_PORT"; 
    let mongo_port: u16;
    
    match env::var(mongo_port_key) {
         Ok(val) => mongo_port = val.parse::<u16>().unwrap(),
         Err(_e) => mongo_port = 27017
    }
    
    let client = Client::connect(&mongo_host, mongo_port)
        .ok().expect("Failed to initialize standalone client.");

    client
}

pub mod repository;