use std::env;
use spawnflake::{
    configuration::{self, config_model::GenericConfiguration},
    databases::my_sql::spawn::spawn,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut items:i32 = 100;

    if args.len() == 1 {
        items = args[1].parse().unwrap();
    }

    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();
    spawn(&config, config.mysql_configuration.schema, items);
}
