use postgres::{Client, Error, NoTls};

use crate::datastores::generic::common_models::{Describe, ForeignKeyRel};

pub fn get_tables(client: &mut Client, schema: String) -> Result<Vec<String>, Error> {
    let mut table_names: Vec<String> = vec![];

    for row in client.query(
        "SELECT distinct(table_name)
        FROM information_schema.columns where table_schema = '$1'",
        &[&schema],
    )? {
        let name: String = row.get(0);
        table_names.push(name);
    }

    return Ok(table_names);
}

pub fn get_foreign_keys(
    client: &mut Client,
    table_name: String,
    schema: String,
) -> Result<Vec<ForeignKeyRel>, Error> {
    let mut foreign_keys: Vec<ForeignKeyRel> = vec![];

    for row in client.query(
        "SELECT
        tc.table_name,
        kcu.column_name,
        ccu.table_name as ref_table,
        ccu.column_name as ref_column
        FROM
        information_schema.table_constraints AS tc
        JOIN information_schema.key_column_usage AS kcu
          ON tc.constraint_name = kcu.constraint_name
          AND tc.table_schema = kcu.table_schema
        JOIN information_schema.constraint_column_usage AS ccu
          ON ccu.constraint_name = tc.constraint_name
          AND ccu.table_schema = tc.table_schema
    WHERE tc.constraint_type = 'FOREIGN KEY' 
    AND tc.table_name='$1'
    AND tc.table_schema='$2';",
        &[&table_name, &schema],
    )? {
        let rkr = ForeignKeyRel {
            table_name: row.get(0),
            column_name: row.get(1),
            referenced_table_name: row.get(2),
            referenced_column_name: row.get(3),
        };
        foreign_keys.push(rkr);
    }

    return Ok(foreign_keys);
}

pub fn get_columns(client: &mut Client, table_name: String) -> Result<Vec<Describe>, Error> {
    todo!()
}