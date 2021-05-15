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
