mod name_generator;
#[macro_use]
mod number_generator;
mod configuration;
use databases::mysql;

use crate::configuration::config_model::GenericConfiguration;
use crate::name_generator::loader::loader;
use crate::name_generator::name::generate_name;
mod databases;

fn main() {
    let config: GenericConfiguration = configuration::reader::read("./config.json").unwrap();

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        &config.mysql_configuration.user,
        &config.mysql_configuration.password,
        &config.mysql_configuration.address,
        &config.mysql_configuration.port,
        &config.mysql_configuration.schema
    );
    // mysql://root:password@localhost:3307/db_name
    let connection = mysql::connection::create_connection(url);

    let firstname = loader(&config, "firstname");
    let lastname = loader(&config, "lastname");

    for _i in 0..10 {
        println!("{} {}", generate_name(&firstname), generate_name(&lastname));
    }
}
