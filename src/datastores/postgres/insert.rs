use postgres::{Client, Error};

pub fn insert_record(
    client: &mut Client,
    table: String,
    columns: String,
    values: String,
) -> Result<i64, Error> {
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({}) RETURNING id",
        table, columns, values
    );

    let id = match client.query_one(&query[..], &[]) {
        Ok(data) => data,
        Err(why) => panic!("{}", why),
    };

    Ok(id.get(0))
}
