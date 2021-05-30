use mysql::{Conn, Opts};

use crate::{
    configuration::config_model::GenericConfiguration,
    databases::{generic::schema::read_schema, my_sql::insert},
    name_generator::{
        loader::{loader, name_generator_exists},
        name::generate_name,
    },
    number_generator::number::{generate_int_number, int_generator_exists},
    random_number,
    string_generator::strings::generate_alphas,
};

use super::discover::ForeignKeyRel;

#[derive(Debug, Clone)]
struct CdDt {
    name: String,
    data_type: String,
    fk: bool,
    dep: Option<ForeignKeyRel>,
}

#[derive(Debug, Clone)]
struct TempKeys {
    id: Vec<i32>,
    table_name: String,
}

pub fn spawn(config: &GenericConfiguration, schema_name: String, no_of_record: i32) {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        &config.mysql_configuration.user,
        &config.mysql_configuration.password,
        &config.mysql_configuration.address,
        &config.mysql_configuration.port,
        &config.mysql_configuration.schema
    );
    let connection_options = match Opts::from_url(&url) {
        Ok(data) => data,
        Err(why) => {
            panic!("{}", why);
        }
    };

    let mut connection = match Conn::new(connection_options) {
        Ok(con) => con,
        Err(why) => {
            panic!("{}", why);
        }
    };

    let schema = read_schema(&mut connection, schema_name);

    let mut temp_keys: Vec<TempKeys> = vec![];
    for table in schema {
        let mut columns: Vec<CdDt> = table
            .clone()
            .fields
            .into_iter()
            .filter(|a| a.extra != "auto_increment")
            .map(|f| {
                let fk_exists = table
                    .clone()
                    .rel
                    .into_iter()
                    .any(|r| r.column_name == f.field);

                let dep = table
                    .clone()
                    .rel
                    .into_iter()
                    .find(|r| r.column_name == f.field);

                return CdDt {
                    name: f.field,
                    data_type: f.data_type,
                    fk: fk_exists,
                    dep: dep,
                };
            })
            .collect();

        columns.sort_by(|a, b| a.fk.cmp(&b.fk));

        println!("{:?}", columns);

        let mut fk_keys: Vec<i32> = vec![];
        for _i in 0..no_of_record {
            let mut values: Vec<String> = vec![];
            let mut fk_table_data;
            for cd in &columns {
                if cd.clone().fk == false {
                    if name_generator_exists(&config, &cd.name) && cd.data_type.contains("varchar")
                    {
                        values.push(format!("'{}'", generate_name(&loader(&config, &cd.name))));
                    } else if cd.data_type.contains("varchar") {
                        values.push(format!("'{}'", generate_alphas(5)));
                    } else if int_generator_exists(&config, &cd.name) && cd.data_type.eq("int") {
                        values.push(format!(
                            "'{}'",
                            generate_int_number(&config, &cd.name).to_string()
                        ));
                    } else if cd.data_type.eq("int") {
                        values.push(format!(
                            "'{}'",
                            random_number!(i32)(0, 2147483647).to_string()
                        ));
                    }
                } else {
                    let fk_table = cd.clone().dep.unwrap().referenced_table_name;
                    fk_table_data = temp_keys.clone()
                        .into_iter()
                        .find(|f| f.table_name == fk_table)
                        .unwrap();
                    let fk_index =random_number!(i32)(0, fk_table_data.id.len() as i32);
                    values.push(format!(
                        "'{}'",
                        fk_table_data.id.get(fk_index as usize).unwrap()
                    ));
                }
            }

            let _r = insert::insert_record(
                &mut connection,
                table.table_name.to_owned(),
                columns
                    .clone()
                    .into_iter()
                    .map(|f| f.name)
                    .collect::<Vec<String>>()
                    .join(","),
                values.join(","),
            );

            let key = insert::last_id(&mut connection);
            fk_keys.push(key);
        }

        temp_keys.push(TempKeys {
            id: fk_keys,
            table_name: table.table_name,
        });
    }
}
