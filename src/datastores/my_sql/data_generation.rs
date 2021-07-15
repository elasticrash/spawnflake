use core::panic;
use std::{
    collections::VecDeque,
    io::{self, Write},
    usize,
};

use mysql::{Conn, Opts};

use crate::{
    byte_generator::bytes::generate_bytes,
    configuration::config_model::GenericConfiguration,
    datastores::{
        datastore::DataGeneration,
        generic::common_models::{CdDt, TableFields, TempKeys},
        my_sql::{
            const_types::const_types,
            discover,
            insert::{self, has_data},
            random_values::{generate_date_time, generate_numeric, select_enum},
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

use super::datastore_models::Mysql;

impl DataGeneration<Conn> for Mysql {
    fn spawn(&mut self, config: &GenericConfiguration, no_of_record: i32) {
        let db_configuration = match &config.mysql_configuration {
            Some(config) => config,
            None => {
                println!("Configuration not found skipping mysql data generator");
                return;
            }
        };

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            &db_configuration.user,
            &db_configuration.password,
            &db_configuration.address,
            &db_configuration.port,
            &db_configuration.schema
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

        <Mysql as DataGeneration<Conn>>::set_schema(
            self,
            &mut connection,
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
                .filter(|a| a.extra == "")
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

                    let ng = table
                        .clone()
                        .fields
                        .into_iter()
                        .any(|r| r.key == "PRI" && r.extra == "");

                    return CdDt {
                        name: f.field,
                        data_type: f.data_type,
                        fk: fk_exists,
                        non_generated: ng,
                        dep: dep,
                    };
                })
                .collect();

            columns.sort_by(|a, b| a.fk.cmp(&b.fk));
            let mut fk_keys: Vec<i64> = vec![];
            for _i in 0..no_of_record {
                print!("*");
                io::stdout().flush();

                let mut values: Vec<String> = vec![];
                let mut fk_table_data;
                for cd in &columns {
                    if cd.clone().fk == false || cd.non_generated == true {
                        let end_bytes = cd.data_type.find("(").unwrap_or(cd.data_type.len());

                        match &cd.data_type[0..end_bytes] {
                            const_types::VARCHAR
                            | const_types::CHAR
                            | const_types::TEXT
                            | const_types::LONG_TEXT
                            | const_types::MEDIUM_TEXT => {
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
                            const_types::BINARY
                            | const_types::VARBINARY
                            | const_types::BLOB
                            | const_types::LONG_BLOB => {
                                values.push(format!("0x{}", generate_bytes(&cd.data_type)));
                            }
                            const_types::INT
                            | const_types::UNSIGNED_INT
                            | const_types::SMALLINT
                            | const_types::UNSINGED_SMALLINT
                            | const_types::TINYINT
                            | const_types::UNSINGED_TINYINT
                            | const_types::MEDIUMINT
                            | const_types::BIGINT
                            | const_types::UNSINGED_BIGINT
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
                                } else if cd.non_generated {
                                    let next_id =
                                        has_data(&mut connection, table.table_name.to_owned()) + 1;
                                    values.push(format!("'{}'", next_id));

                                    fk_keys.push(next_id);
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
                                values.push(format!(
                                    "'{}'",
                                    generate_date_time(&cd.data_type).unwrap()
                                ));
                            }
                            const_types::BIT => {
                                values.push(format!("{}", random_number!(i8)(0, 2).to_string()));
                            }
                            const_types::ENUM => {
                                values.push(format!("{}", select_enum(&cd.data_type).unwrap()));
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
                        let fk_index = random_number!(i64)(0, fk_table_data.id.len() as i64);
                        values.push(format!(
                            "'{}'",
                            fk_table_data.id.get(fk_index as usize).unwrap()
                        ));
                    }
                }

                let _r = insert::insert_record(
                    &mut connection,
                    table.table_name.to_owned(),
                    columns
                        .clone()
                        .into_iter()
                        .map(|f| format!("`{}`", f.name))
                        .collect::<Vec<String>>()
                        .join(","),
                    values.join(","),
                );
                if !(columns.clone().into_iter().any(|f| f.non_generated == true)) {
                    let key = insert::last_id(&mut connection);
                    fk_keys.push(key);
                }
            }

            println!(
                "table {} has {} records",
                table.table_name,
                has_data(&mut connection, table.table_name.to_owned())
            );

            temp_keys.push(TempKeys {
                id: fk_keys,
                table_name: table.clone().table_name,
            });
        }
    }
    fn set_schema(&mut self, conn: &mut Conn, schema: &String) {
        let tables = discover::get_tables(conn, schema.clone());

        for t in tables.unwrap() {
            let fields = discover::get_columns(conn, t.to_string());
            let get_foreign_keys = discover::get_foreign_keys(conn, t.to_string(), schema.clone());

            self.schema.push(TableFields {
                table_name: t.clone(),
                fields: fields.unwrap(),
                rel: get_foreign_keys.unwrap_or(vec![]),
            })
        }

        self.schema.sort_by(|a, b| a.rel.len().cmp(&b.rel.len()));

        let mut tree: VecDeque<TableFields> = VecDeque::new();
        let mut current = 0;
        for (i, tf) in self.schema.clone().into_iter().enumerate() {
            let number_of_foreign_keys = tf.clone().rel.into_iter().fold(0, |acc, x| {
                if x.table_name == tf.table_name {
                    acc + 1
                } else {
                    acc
                }
            });

            if tf.rel.len() == 0 || number_of_foreign_keys == 0 {
                tree.push_front(tf);
                current += 1;
            } else {
                let occurance: i32 = tf.clone().rel.into_iter().fold(0, |acc, x| {
                    if tree
                        .clone()
                        .into_iter()
                        .any(|b| b.table_name == x.referenced_table_name)
                    {
                        acc + 1
                    } else {
                        acc
                    }
                });

                tree.push_back(tf.clone());

                let tree_size = tree.clone().len();

                if !(occurance == tf.rel.len() as i32) {
                    tree.swap(current, tree_size - 1);
                }
            }
        }
        self.schema = tree.into_iter().collect();

        println!("{}", self);
    }
    fn new() -> Self {
        let table_fields: Vec<TableFields> = vec![];
        Self {
            schema: table_fields,
        }
    }
}
