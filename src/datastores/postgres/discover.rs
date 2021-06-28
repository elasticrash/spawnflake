use postgres::{Client, Error};

use crate::datastores::generic::common_models::{Describe, ForeignKeyRel};

pub fn get_tables(client: &mut Client, schema: String) -> Result<Vec<String>, Error> {
    let mut table_names: Vec<String> = vec![];
    for row in client.query(
        "SELECT distinct(table_name)
        FROM information_schema.columns where table_schema = $1",
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
    AND tc.table_name=$1
    AND tc.table_schema=$2;",
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

pub fn get_columns(
    client: &mut Client,
    table_name: String,
    schema: String,
) -> Result<Vec<Describe>, Error> {
    let mut primary_keys: Vec<String> = vec![];
    let mut column_names: Vec<Describe> = vec![];

    for row in client.query(
        "select c.column_name
        from information_schema.table_constraints tco
        join information_schema.key_column_usage c
             on c.constraint_name = tco.constraint_name
             and c.constraint_schema = tco.constraint_schema
             and c.constraint_name = tco.constraint_name
        where tco.constraint_type = 'PRIMARY KEY'
        and c.table_name = $1 and c.table_schema = $2;",
        &[&table_name, &schema],
    )? {
        let key: String = row.get(0);
        primary_keys.push(key);
    }

    for row in client.query(
        "SELECT column_name, data_type, is_nullable, column_default
        FROM information_schema.columns where
         table_name = $1 and table_schema = $2;",
        &[&table_name, &schema],
    )? {
        let column_name: String = row.get(0);
        let data_type: String = row.get(1);
        let is_nullable: String = row.get(2);
        let column_default: Option<String> = row.get(3);

        column_names.push(Describe {
            field: column_name,
            data_type: data_type,
            null: is_nullable,
            key: if primary_keys.iter().any(|x| {
                let rec: String = row.get(0);
                return **x == rec;
            }) {
                "PRI".to_string()
            } else {
                "".to_string()
            },
            default: None,
            extra: column_default.unwrap_or("null".to_string()),
        });
    }
    return Ok(column_names);
}
