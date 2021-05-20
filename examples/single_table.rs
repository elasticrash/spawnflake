extern crate spawnflake;

use spawnflake::{
    configuration::{self, config_model::GenericConfiguration},
    databases::my_sql::spawn::spawn,
};

fn main() {
    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();
    spawn(&config, "test".to_string(), 99);
}
