use postgres::{Client, Error, NoTls};

pub fn insert_record(
    client: &mut Client,
    table: String,
    columns: String,
    values: String,
) -> Result<i32, Error> {
    let id = client.query_one(
        "INSERT INTO $1 ($2) VALUES ($3) RETURNING id",
        &[&table, &columns, &values])?;
    Ok(id.get(0))
}