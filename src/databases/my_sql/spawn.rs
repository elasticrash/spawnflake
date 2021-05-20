use mysql::{Conn, Opts};

use crate::{
    configuration::config_model::GenericConfiguration,
    databases::{generic::schema::read_schema, my_sql::insert},
    name_generator::{
        loader::{generator_exists, loader},
        name::generate_name,
    },
    random_number,
    string_generator::strings::generate_alphas,
};

#[derive(Debug, Clone)]
struct CdDt {
    name: String,
    data_type: String,
}

pub fn spawn(config: &GenericConfiguration, schema_name: String, no_of_record:i32) {
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

    let schema = read_schema(&mut connection, schema_name);

    for table in schema {
        let columns: Vec<CdDt> = table
            .clone()
            .fields
            .into_iter()
            .filter(|a| a.extra != "auto_increment")
            .map(|f| CdDt {
                name: f.field,
                data_type: f.data_type,
            })
            .collect();

        for _i in 0..no_of_record {
            let mut values: Vec<String> = vec![];

            for cd in &columns {
                if generator_exists(&config, &cd.name) && cd.data_type.contains("varchar") {
                    values.push(format!("'{}'", generate_name(&loader(&config, &cd.name))));
                } else if cd.data_type.contains("varchar") {
                    values.push(format!("'{}'", generate_alphas(5)));
                }

                if cd.data_type.eq("int") {
                    values.push(format!("'{}'", random_number!(i32)(18, 78).to_string()));
                }
            }

            let _r = insert::insert_record(
                &mut connection,
                table.table_name.to_owned(),
                columns
                    .clone()
                    .into_iter()
                    .map(|f| f.name)
                    .collect::<Vec<String>>()
                    .join(","),
                values.join(","),
            );
        }
    }
}
