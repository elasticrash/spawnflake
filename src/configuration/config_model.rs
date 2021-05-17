use serde::Deserialize;

/// Top level configuration class
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GenericConfiguration {
    pub name_generators: Vec<NameGenerator>,
    pub mysql_configuration: MySQLConfiguration,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NameGenerator {
    pub name: String,
    pub rules: Vec<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct MySQLConfiguration {
    pub address: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub schema: String,
}