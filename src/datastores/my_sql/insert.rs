use mysql::prelude::*;
use mysql::{Conn, Error};

pub fn insert_record(
    conn: &mut Conn,
    table: String,
    columns: String,
    values: String,
) -> Result<(), Error> {
    conn.exec_drop(
        format!("INSERT INTO {} ({}) VALUES ({})", table, columns, values),
        (),
    )?;

    Ok(())
}

pub fn last_id(conn: &mut Conn) -> i64 {
    let id: Result<Vec<i64>, Error> =
        conn.query_map("SELECT LAST_INSERT_ID()".to_string(), |id| id);

    *id.unwrap().first().unwrap()
}
