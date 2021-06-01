use std::env;
use spawnflake::{
    configuration::{self, config_model::GenericConfiguration},
    databases::my_sql::spawn::spawn,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut schema:String = "default".to_string();
    let mut items:i32 = args[2].parse().unwrap_or(100);

    if args.len() == 1 {
        schema = args[1].parse().unwrap();
    }

    if args.len() > 2 {
        items = args[2].parse().unwrap();
    }

    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();
    spawn(&config, schema, items);
}
