use mysql::Conn;

use crate::databases::my_sql::discover::{self, Describe};

#[derive(Debug, Clone)]
pub struct TableFields {
    pub table_name: String,
    pub fields: Vec<Describe>,
}

pub fn read_schema(conn: &mut Conn, schema: String) -> Vec<TableFields> {
    let mut table_fields: Vec<TableFields> = vec![];
    let tables = discover::get_tables(conn, schema);

    for t in tables.unwrap() {
        let fields = discover::get_columns(conn, t.to_string());
        table_fields.push(TableFields {
            table_name: t,
            fields: fields.unwrap(),
        })
    }

    table_fields
}
