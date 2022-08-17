use serde::Deserialize;

/// Top level configuration class
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GenericConfiguration {
    pub types: ValueTypes,
    pub mysql_configuration: Option<RelationalDatabaseConfiguration>,
    pub postgres_configuration: Option<RelationalDatabaseConfiguration>,
    pub table_capacity: Option<Vec<Capacity>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ValueTypes {
    pub string: Vec<Patterns<Vec<String>>>,
    pub integer: Vec<Patterns<i32>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Patterns<T> {
    pub name: String,
    pub rules: Vec<T>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Capacity {
    pub table: String,
    pub capacity: i32,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct RelationalDatabaseConfiguration {
    pub address: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub schema: String,
}
