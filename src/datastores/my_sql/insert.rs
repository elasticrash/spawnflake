use mysql::prelude::*;
use mysql::{Conn, Error};

pub fn insert_record(
    conn: &mut Conn,
    table: String,
    columns: String,
    values: String,
) -> Result<(), Error> {
    conn.exec_drop(
        format!("INSERT INTO {} ({}) VALUES ({})", &table, &columns, &values),
        (),
    )?;

    Ok(())
}

pub fn last_id(conn: &mut Conn) -> i64 {
    let id: Result<Vec<i64>, Error> =
        conn.query_map("SELECT LAST_INSERT_ID()", |id| id);

    *id.unwrap().first().unwrap()
}

pub fn has_data(conn: &mut Conn, table: String) -> i64 {
    let count: Result<Vec<i64>, Error> =
        conn.query_map(format!("SELECT COUNT(1) as c FROM {table}"), |c| c);

    *count.unwrap().first().unwrap()
}
