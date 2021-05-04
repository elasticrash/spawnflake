use super::{chain::Chain, configuration::config_model::GeneratorConfiguration};

pub fn loader(config: &GeneratorConfiguration) -> Vec<Chain> {
    let mut chains: Vec<Chain> = vec![];
    for rule in &config.name_generators[0].rules {
        let chain = Chain::new(rule.to_vec());
        chains.push(chain);
    }

    chains
}
