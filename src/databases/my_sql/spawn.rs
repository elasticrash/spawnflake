use std::io::{self, Write};

use mysql::{Conn, Opts};

use crate::{
    byte_generator::bytes::generate_bytes,
    configuration::config_model::GenericConfiguration,
    databases::{
        generic::{const_types::const_types, schema::read_schema},
        my_sql::{
            data_types::{
                check_if_binary, check_if_date_time, check_if_numeric, check_if_string,
                generate_date_time, generate_numeric,
            },
            insert,
        },
    },
    name_generator::{
        loader::{loader, name_generator_exists},
        name::generate_name,
    },
    number_generator::number::{generate_int_number, int_generator_exists},
    random_number,
    string_generator::strings::generate_alphas,
};

use super::discover::ForeignKeyRel;

#[derive(Debug, Clone)]
struct CdDt {
    name: String,
    data_type: String,
    fk: bool,
    dep: Option<ForeignKeyRel>,
}

#[derive(Debug, Clone)]
struct TempKeys {
    id: Vec<i32>,
    table_name: String,
}

#[allow(unused_must_use)]
pub fn spawn(config: &GenericConfiguration, no_of_record: i32) {
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

    let schema = read_schema(&mut connection, config.clone().mysql_configuration.schema);
    let mut temp_keys: Vec<TempKeys> = vec![];
    for table in schema {
        println!("");
        println!("* Generating records for {:?}", &table.table_name);

        let mut columns: Vec<CdDt> = table
            .clone()
            .fields
            .into_iter()
            .filter(|a| a.extra != "auto_increment")
            .map(|f| {
                let fk_exists = table
                    .clone()
                    .rel
                    .into_iter()
                    .any(|r| r.column_name == f.field);

                let dep = table
                    .clone()
                    .rel
                    .into_iter()
                    .find(|r| r.column_name == f.field);

                return CdDt {
                    name: f.field,
                    data_type: f.data_type,
                    fk: fk_exists,
                    dep: dep,
                };
            })
            .collect();

        columns.sort_by(|a, b| a.fk.cmp(&b.fk));
        let mut fk_keys: Vec<i32> = vec![];
        for _i in 0..no_of_record {
            print!("*");
            io::stdout().flush();

            let mut values: Vec<String> = vec![];
            let mut fk_table_data;
            for cd in &columns {
                if cd.clone().fk == false {
                    match &cd.data_type[..] {
                        const_types::VARCHAR | const_types::CHAR | const_types::TEXT => {
                            if name_generator_exists(&config, &cd.name)
                                && cd.data_type.contains(const_types::VARCHAR)
                            {
                                values.push(format!(
                                    "'{}'",
                                    generate_name(&loader(&config, &cd.name))
                                ));
                            } else {
                                values.push(format!("'{}'", generate_alphas(&cd.data_type)));
                            }
                        }
                        const_types::BINARY | const_types::BLOB => {
                            values.push(format!("0x{}", generate_bytes(&cd.data_type)));
                        }
                        const_types::INT
                        | const_types::SMALLINT
                        | const_types::TINYINT
                        | const_types::MEDIUMINT
                        | const_types::BIGINT
                        | const_types::DECIMAL
                        | const_types::FLOAT
                        | const_types::DOUBLE => {
                            if int_generator_exists(&config, &cd.name)
                                && cd.data_type.eq(const_types::INT)
                            {
                                values.push(format!(
                                    "'{}'",
                                    generate_int_number(&config, &cd.name).to_string()
                                ));
                            } else {
                                values.push(format!(
                                    "'{}'",
                                    generate_numeric(&cd.data_type).unwrap_or("0".to_string())
                                ));
                            }
                        }
                        const_types::DATETIME
                        | const_types::DATE
                        | const_types::TIMESTAMP
                        | const_types::TIME
                        | const_types::YEAR => {
                            values
                                .push(format!("'{}'", generate_date_time(&cd.data_type).unwrap()));
                        }
                        const_types::BIT => {
                            values.push(format!("{}", random_number!(i8)(0, 2).to_string()));
                        }
                        _ => println!("type {} not currently supported", cd.data_type),
                    }
                } else {
                    let fk_table = cd.clone().dep.unwrap().referenced_table_name;
                    fk_table_data = temp_keys
                        .clone()
                        .into_iter()
                        .find(|f| f.table_name == fk_table)
                        .unwrap();
                    let fk_index = random_number!(i32)(0, fk_table_data.id.len() as i32);
                    values.push(format!(
                        "'{}'",
                        fk_table_data.id.get(fk_index as usize).unwrap()
                    ));
                }
            }

            let _r = insert::insert_record(
                &mut connection,
                table.table_name.to_owned(),
                columns // TODO: change this to supported types only
                    .clone()
                    .into_iter()
                    .map(|f| f.name)
                    .collect::<Vec<String>>()
                    .join(","),
                values.join(","),
            );

            let key = insert::last_id(&mut connection);
            fk_keys.push(key);
        }

        temp_keys.push(TempKeys {
            id: fk_keys,
            table_name: table.table_name,
        });
    }
}
