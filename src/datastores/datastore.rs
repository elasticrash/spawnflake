use mysql::Conn;
use crate::configuration::config_model::GenericConfiguration;

pub trait DataGeneration {
    fn spawn(&mut self, config: &GenericConfiguration, no_of_record: i32);
    fn set_schema(&mut self, conn: &mut Conn, schema: &String);
    fn new()-> Self;
}