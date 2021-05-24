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

pub fn get_foreign_keys(conn: &mut Conn, schema: String) -> Result<Vec<ForeignKeyRel>, Error> {
    return conn.query_map(
        format!(
            "SELECT
                TABLE_NAME,
                COLUMN_NAME,
                REFERENCED_TABLE_NAME,
                REFERENCED_COLUMN_NAME FROM information_schema.KEY_COLUMN_USAGE where TABLE_SCHEMA ='{}' AND REFERENCED_TABLE_NAME is not null",
            schema
        ),
        |(table_name,column_name,referenced_table_name,referenced_column_name )| ForeignKeyRel{
            table_name,column_name,referenced_table_name,referenced_column_name
        },
    );
}

pub fn get_columns(conn: &mut Conn, table_name: String) -> Result<Vec<Describe>, Error> {
    return conn.query_map(
        format!("DESCRIBE {}", table_name),
        |(field, data_type, null, key, default, extra)| Describe {
            field,
            data_type,
            null,
            key,
            default,
            extra,
        },
    );
}

#[derive(Debug, Clone)]
pub struct Describe {
    pub field: String,
    pub data_type: String,
    pub null: String,
    pub key: String,
    pub default: Option<String>,
    pub extra: String,
}

#[derive(Debug, Clone)]
pub struct ForeignKeyRel {
    pub table_name: String,
    pub column_name: String,
    pub referenced_table_name: String,
    pub referenced_column_name: String,
}
