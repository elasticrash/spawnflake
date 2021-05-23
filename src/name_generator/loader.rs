use core::panic;
use crate::configuration::config_model::GenericConfiguration;
use super::chain::Chain;

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