use std::io::{self, Write};

use crate::{byte_generator::bytes::generate_bytes, configuration::config_model::GenericConfiguration, datastores::{datastore::DataGeneration, generic::common_models::{CdDt, TableFields, TempKeys}, postgres::{const_types::const_types, discover, insert, random_values::{generate_date_time, generate_numeric}}}, name_generator::{
        loader::{loader, name_generator_exists},
        name::generate_name,
    }, number_generator::number::{generate_int_number, int_generator_exists}, random_number, string_generator::strings::generate_alphas};

use super::datastore_models::Postgres;
use postgres::{Client, NoTls};

impl DataGeneration<Client> for Postgres {
    fn spawn(&mut self, config: &GenericConfiguration, no_of_record: i32) {
        let db_configuration = match &config.postgres_configuration {
            Some(config) => config,
            None => {
                println!("Configuration not found skipping postgres data generator");
                return;
            }
        };

        let connection_string = format!(
            "host={} user={} password={} port={}",
            &db_configuration.address,
            &db_configuration.user,
            &db_configuration.password,
            &db_configuration.port,
        );

        let mut client = match Client::connect(connection_string.as_str(), NoTls) {
            Ok(data) => data,
            Err(why) => {
                panic!("{}", why);
            }
        };

        <Postgres as DataGeneration<Client>>::set_schema(
            self,
            &mut client,
            &db_configuration.schema,
        );

        let mut temp_keys: Vec<TempKeys> = vec![];
        for table in &self.schema {
            println!("");
            println!("* Generating records for {:?}", &table.table_name);

            let mut columns: Vec<CdDt> = table
                .clone()
                .fields
                .into_iter()
                .filter(|a| a.key != "PRI")
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
                        let end_bytes = cd.data_type.find("(").unwrap_or(cd.data_type.len());

                        match &cd.data_type[0..end_bytes] {
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
                            const_types::BYTE => {
                                values.push(format!("0x{}", generate_bytes(&cd.data_type)));
                            }
                            const_types::INT
                            | const_types::SMALLINT
                            | const_types::BIGINT
                            | const_types::DECIMAL
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
                            const_types::DATE
                            | const_types::TIMESTAMP
                            | const_types::TIME
                            | const_types::INTERVAL => {
                                values.push(format!(
                                    "'{}'",
                                    generate_date_time(&cd.data_type).unwrap()
                                ));
                            }
                            const_types::BOOLEAN => {
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

                let key = insert::insert_record(
                    &mut client,
                    table.table_name.to_owned(),
                    columns // TODO: change this to supported types only
                        .clone()
                        .into_iter()
                        .map(|f| f.name)
                        .collect::<Vec<String>>()
                        .join(","),
                    values.join(","),
                );

                fk_keys.push(key.unwrap());
            }

            temp_keys.push(TempKeys {
                id: fk_keys,
                table_name: table.clone().table_name,
            });
        }
    }

    fn set_schema(&mut self, client: &mut Client, schema: &String) {
        let tables = discover::get_tables(client, schema.clone());
        for t in tables.unwrap() {
            let fields = discover::get_columns(client, t.to_string(), schema.to_string());
            let get_foreign_keys =
                discover::get_foreign_keys(client, t.to_string(), schema.clone());

            self.schema.push(TableFields {
                table_name: t.clone(),
                fields: fields.unwrap(),
                rel: get_foreign_keys.unwrap_or(vec![]),
            })
        }
    }

    fn new() -> Self {
        let table_fields: Vec<TableFields> = vec![];
        Self {
            schema: table_fields,
        }
    }
}
