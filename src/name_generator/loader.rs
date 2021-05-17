use crate::GenericConfiguration;
use core::panic;

use super::{chain::Chain};

pub fn loader(config: &GenericConfiguration, name: &str) -> Vec<Chain> {
    let mut chains: Vec<Chain> = vec![];
    let name_generator = &config.name_generators.iter().find(|x| x.name == name);
    match name_generator {
        Some(data) => {
            for rule in &data.rules {
                let chain = Chain::new(rule.to_vec());
                chains.push(chain);
            }
        }
        None => panic!("no such name for a generator exists")
    }

    chains
}
