use clap::{App, Arg};
use spawnflake::{configuration::{self, config_model::GenericConfiguration}, datastores::{datastore::DataGeneration, my_sql::datastore_models::Mysql, postgres::datastore_models::Postgres}};

fn main() {
    let matches = App::new("Spawnflake CLI")
    .version("0.2.0")
    .author("Stefanos Kouroupis. <manekato@gmail.com>")
    .about("Generates random data")
    .arg(Arg::with_name("spawn-size")
         .short("s")
         .long("spawn-size")
         .help("Set the number of rows per table to be generated")
         .takes_value(true))
    .arg(Arg::with_name("datastore")
         .short("d")
         .help("Sets the type of the datastore to generate values. By default executes the entire configuration")
         .takes_value(true))
    .arg(Arg::with_name("configuration")
         .short("c")
         .help("sets a custom configuration name and path relative to this location")
         .takes_value(true))
    .get_matches();

    
    let configuration_file: &str = matches
    .value_of("configuration")
    .unwrap_or("./config.json");

    let config: GenericConfiguration = configuration::reader::read(configuration_file).unwrap();
    let size: i32 = matches
        .value_of("spawn-size")
        .unwrap_or("100")
        .parse()
        .unwrap();
    let datastore: &str = matches
        .value_of("datastore")
        .unwrap_or("all");


    if datastore == "all" {
        if config.mysql_configuration != None {
            println!("generating data for mysql datastore");
            let mut db = Mysql::new();
            db.spawn(&config, size);
        }
        if config.postgres_configuration != None {
            println!("generating data for postgres datastore");
            let mut db = Postgres::new();
            db.spawn(&config, 10);
        }
    }

    if datastore  == "mysql"{
        if config.mysql_configuration != None {
            let mut db = Mysql::new();
            db.spawn(&config, size);
        } else{
            println!("mysql Configuration is missing");
        }
    }

    if datastore  == "postgres"{
        if config.postgres_configuration != None {
            let mut db = Postgres::new();
            db.spawn(&config, 10);
        } else{
            println!("postgres Configuration is missing");
        }
    }
}
