extern crate spawnflake;

use spawnflake::{configuration::{self, config_model::GenericConfiguration}, datastores::{datastore::DataGeneration, my_sql::datastore_models::Mysql}};

fn main() {
    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();    
    let mut db = Mysql::new();
    db.spawn(&config,  10);
}
