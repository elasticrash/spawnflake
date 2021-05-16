mod name_generator;
#[macro_use]
mod number_generator;
mod configuration;

use mysql::{Conn, Opts};

use crate::configuration::config_model::GenericConfiguration;
use crate::{databases::generic::schema::read_schema, name_generator::name::generate_name};
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

    let schema = read_schema(&mut connection, "test".to_string());
    println!("schema {:?}", schema);

    let firstname = loader(&config, "firstname");
    let lastname = loader(&config, "lastname");

    for table in schema {
        let columns: Vec<String> = table
        .clone()
        .fields
        .into_iter()
        .filter(|a| a.extra != "auto_increment")
        .map(|f| f.field)
        .collect();

        for _i in 0..100 {
            let _r = insert::insert_record(
                &mut connection,
                table.table_name.to_owned(),
                columns.join(","),
                format!(
                    "'{}','{}',{}",
                    generate_name(&firstname),
                    generate_name(&lastname),
                    random_number!(i32)(18, 78)
                ),
            );
        }
    }
}
