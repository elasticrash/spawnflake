use std::env;
use spawnflake::{
    configuration::{self, config_model::GenericConfiguration},
    databases::my_sql::spawn::spawn,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut items:i32 = 100;
    println!("{:?}", args);

    if args.len() > 1 {
        items = args[1].parse().unwrap();
    }
    
    println!("{}", items);

    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();
    spawn(&config, items);
}
