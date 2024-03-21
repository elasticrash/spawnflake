use crate::configuration::config_model::GenericConfiguration;
use crate::random_number;

use super::chain::Chain;
use super::loader::loader;

/// generates a name based on chains
pub fn generate_name(config: &GenericConfiguration, name: &str) -> String {
    let chain = loader(config, &name);
    generate_name_from_chain(&chain)
}

/// generates a name based on chains
pub fn generate_name_from_chain(chain: &Vec<Chain>) -> String {
    let mut name_parts: Vec<String> = vec![];
    for (i, c) in chain.iter().enumerate() {
        let token = random_number!(usize)(0, c.tokens.len());

        if i == 0 {
            name_parts.push(c.tokens[token].to_owned())
        } else if i == chain.len() - 1 {
            name_parts.push(c.tokens[token].to_owned());
        } else {
            let skip = random_number!(usize)(0, 2);

            if skip == 1 {
                let token = random_number!(usize)(0, c.tokens.len());
                name_parts.push(c.tokens[token].to_owned());
            }
        }
    }
    name_parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::name_generator::chain::*;
    use crate::name_generator::name::generate_name_from_chain;
    #[test]
    fn get_name() {
        let first = Chain::new(vec!["Aa".to_string()]);
        let second = Chain::new(vec!["Bb".to_string()]);

        assert_eq!(generate_name_from_chain(&vec![first, second]), "AaBb");
    }
}
