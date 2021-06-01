use mysql::Conn;

use crate::databases::my_sql::discover::{self, Describe, ForeignKeyRel};

#[derive(Debug, Clone)]
pub struct TableFields {
    pub table_name: String,
    pub fields: Vec<Describe>,
    pub rel: Vec<ForeignKeyRel>,
}

pub fn read_schema(conn: &mut Conn, schema: String) -> Vec<TableFields> {
    let mut table_fields: Vec<TableFields> = vec![];
    let tables = discover::get_tables(conn, schema.clone());

    for t in tables.unwrap() {
        let fields = discover::get_columns(conn, t.to_string());
        let get_foreign_keys = discover::get_foreign_keys(conn, t.to_string(), schema.clone());
        
        table_fields.push(TableFields {
            table_name: t.clone(),
            fields: fields.unwrap(),
            rel: get_foreign_keys.unwrap_or(vec![]),
        })
    }

    table_fields
}
