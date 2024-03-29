use crate::datastores::generic::common_models::{Describe, ForeignKeyRel};
use mysql::prelude::*;
use mysql::{Conn, Error};

pub fn get_tables(conn: &mut Conn, schema: String) -> Result<Vec<String>, Error> {
    conn.query_map(
        format!("SELECT TABLE_NAME FROM information_schema.tables where TABLE_SCHEMA = '{schema}'"),
        |table_name| table_name,
    )
}

pub fn get_foreign_keys(
    conn: &mut Conn,
    table_name: String,
    schema: String,
) -> Result<Vec<ForeignKeyRel>, Error> {
    conn.query_map(
        format!(
            "SELECT
                TABLE_NAME,
                COLUMN_NAME,
                REFERENCED_TABLE_NAME,
                REFERENCED_COLUMN_NAME 
                FROM information_schema.KEY_COLUMN_USAGE
                WHERE TABLE_SCHEMA ='{schema}'
                AND (TABLE_NAME = '{table_name}' OR REFERENCED_TABLE_NAME ='{table_name}')
                AND REFERENCED_TABLE_NAME is not null"
        ),
        |(table_name, column_name, referenced_table_name, referenced_column_name)| ForeignKeyRel {
            table_name,
            column_name,
            referenced_table_name,
            referenced_column_name,
        },
    )
}

pub fn get_columns(conn: &mut Conn, table_name: String) -> Result<Vec<Describe>, Error> {
    conn.query_map(
        format!("DESCRIBE {table_name}"),
        |(field, data_type, null, key, default, extra)| Describe {
            field,
            data_type,
            null,
            key,
            default,
            extra,
        },
    )
}
