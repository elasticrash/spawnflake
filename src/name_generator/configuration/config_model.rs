use serde::Deserialize;

/// Top level configuration class
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct GeneratorConfiguration {
    pub name_generators: Vec<NameGenerator>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct NameGenerator {
    pub name: String,
    pub rules: Vec<Vec<String>>,
}