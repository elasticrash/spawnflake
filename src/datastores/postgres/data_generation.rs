use crate::{configuration::config_model::GenericConfiguration, datastores::datastore::DataGeneration};

use super::datastore_models::Postgres;
use postgres::{Client, NoTls};

impl DataGeneration<Client> for Postgres {
    fn spawn(&mut self, config: &GenericConfiguration, no_of_record: i32) {
        todo!()
    }

    fn set_schema(&mut self, conn: &mut Client, schema: &String) {
        todo!()
    }

    fn new()-> Self {
        todo!()
    }
}