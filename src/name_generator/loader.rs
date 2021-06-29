use super::chain::Chain;
use crate::configuration::config_model::GenericConfiguration;
use core::panic;

/// Looks up chain generators (usually for names and addresses)
/// panics if a generator does not exist
pub fn loader(config: &GenericConfiguration, name: &str) -> Vec<Chain> {
    let mut chains: Vec<Chain> = vec![];
    let name_generator = &config.types.string.iter().find(|x| x.name == name);
    match name_generator {
        Some(data) => {
            for rule in &data.rules {
                let chain = Chain::new(rule.to_vec());
                chains.push(chain);
            }
        }
        None => panic!("no such name for a generator exists"),
    }

    chains
}

/// Checks if a name generator exists
pub fn name_generator_exists(config: &GenericConfiguration, name: &str) -> bool {
    let name_generator = &config.types.string.iter().find(|x| x.name == name);

    match name_generator {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        configuration::config_model::{
            GenericConfiguration, Patterns, RelationalDatabaseConfiguration, ValueTypes,
        },
        name_generator::loader::name_generator_exists,
    };

    #[test]
    fn generator_exists() {
        let exists = name_generator_exists(
            &GenericConfiguration {
                types: ValueTypes {
                    string: vec![Patterns::<Vec<String>> {
                        name: "test".to_string(),
                        rules: vec![],
                    }],
                    integer: vec![],
                },
                mysql_configuration: Some(RelationalDatabaseConfiguration {
                    address: "".to_string(),
                    port: 123,
                    user: "".to_string(),
                    password: "".to_string(),
                    schema: "".to_string(),
                }),
                postgres_configuration: None,
            },
            "test",
        );

        assert_eq!(exists, true);
    }

    #[test]
    fn generator_doesnot_exists() {
        let exists = name_generator_exists(
            &GenericConfiguration {
                types: ValueTypes {
                    string: vec![Patterns::<Vec<String>> {
                        name: "random".to_string(),
                        rules: vec![],
                    }],
                    integer: vec![],
                },
                mysql_configuration: Some(RelationalDatabaseConfiguration {
                    address: "".to_string(),
                    port: 123,
                    user: "".to_string(),
                    password: "".to_string(),
                    schema: "".to_string(),
                }),
                postgres_configuration: None,
            },
            "test",
        );

        assert_eq!(exists, false);
    }
}
