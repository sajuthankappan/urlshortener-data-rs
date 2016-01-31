extern crate mongodb;

use bson::Bson;
use mongodb::{ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::FindOneAndUpdateOptions;
use mongodb::coll::options::ReturnDocument;

pub struct CounterRepository;

impl CounterRepository {
    pub fn new() -> CounterRepository {
        CounterRepository
    }

    pub fn increment_counter(&self, counter_type: &str) -> usize {
        let coll = self.get_counter_collection();
        let filter = doc! { "_id" => counter_type};
        let update = doc! { "$inc" => { "count" => 1 } };
        let mut options = FindOneAndUpdateOptions::new();
        options.return_document = ReturnDocument::After;
        let result = coll.find_one_and_update(filter, update, Some(options))
            .ok().expect("Failed to execute find_one_and_update command for increment_counter.");

        let res = result.clone();


        match result.unwrap().get("count") {
            Some(&Bson::FloatingPoint (ref c)) => return c.clone() as usize,
            _ => panic!("Expected Bson::FloatingPoint! {:?}", res),
        }
    }

    fn get_counter_collection(&self) -> mongodb::coll::Collection {
        let client = super::super::get_mongo_client();
        let coll = client.db("urlshortener").collection("counters");
        coll
    }
}
