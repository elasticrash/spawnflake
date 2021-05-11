use mysql::prelude::*;
use mysql::{Conn, Error};

pub fn get_tables(conn: &mut Conn, schema: String) -> Result<Vec<String>, Error> {
    return conn.query_map(
        format!(
            "SELECT TABLE_NAME FROM information_schema.tables where TABLE_SCHEMA = '{}'",
            schema
        ),
        |table_name| table_name,
    );
}

pub fn get_columns(conn: &mut Conn, table_name: String) -> Result<Vec<Describe>, Error> {
    return conn.query_map(
        format!("DESCRIBE {}", table_name),
        |(Field, Type, Null, Key, Default, Extra)| Describe {
            Field,
            Type,
            Null,
            Key,
            Default,
            Extra,
        },
    );
}

#[derive(Debug)]
pub struct Describe {
    Field: String,
    Type: String,
    Null: String,
    Key: String,
    Default: Option<String>,
    Extra: String,
}
