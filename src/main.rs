mod name_generator;
#[macro_use]
mod number_generator;
mod configuration;

use mysql::{Conn, Opts};

use crate::name_generator::name::generate_name;
use crate::{configuration::config_model::GenericConfiguration, databases::my_sql::discover};
use crate::{databases::my_sql::insert, name_generator::loader::loader};
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
    let connection_options = match Opts::from_url(&url) {
        Ok(data) => data,
        Err(why) => {
            panic!("{}", why);
        }
    };

    let mut connection = match Conn::new(connection_options) {
        Ok(con) => con,
        Err(why) => {
            panic!("{}", why);
        }
    };

    let tables = discover::get_tables(&mut connection, "test".to_string());
    println!("available tables {:?}", tables);

    let fields = discover::get_columns(
        &mut connection,
        tables.unwrap().first().unwrap().to_string(),
    );
    println!("available fields {:?}", fields);

    let firstname = loader(&config, "firstname");
    let lastname = loader(&config, "lastname");

    for _i in 0..10 {
        let _r = insert::insert_record(
            &mut connection,
            "users".to_string(),
            "firstname,lastname,age".to_string(),
            format!(
                "'{}','{}',{}",
                generate_name(&firstname),
                generate_name(&lastname),
                random_number!(i32)(18, 78)
            ),
        );
    }
}
