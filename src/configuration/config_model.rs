use serde::Deserialize;

/// Top level configuration class
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GenericConfiguration {
    pub types: ValueTypes,
    pub mysql_configuration: Option<RelationalDatabaseConfiguration>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Default)]
pub struct ValueTypes {
    #[serde(default)]
    pub string: Vec<Patterns<Vec<String>>>,
    #[serde(default)]
    pub integer: Vec<Patterns<i32>>,
    #[serde(default)]
    pub float: Vec<Patterns<f32>>,
    #[serde(default)]
    pub datetime: Vec<Patterns<String>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Patterns<T> {
    pub name: String,
    pub rules: Vec<T>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct RelationalDatabaseConfiguration {
    pub address: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub schema: String,
}
