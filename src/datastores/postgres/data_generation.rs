use crate::{
    configuration::config_model::GenericConfiguration,
    datastores::{
        datastore::DataGeneration, generic::common_models::TableFields, postgres::discover,
    },
};

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

        todo!()
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
