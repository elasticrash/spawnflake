extern crate spawnflake;

use spawnflake::{configuration::{self, config_model::GenericConfiguration}, datastores::{datastore::DataGeneration, postgres::datastore_models::Postgres}};

fn main() {
    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();    
    let mut db = Postgres::new();
    db.spawn(&config,  10);
}
