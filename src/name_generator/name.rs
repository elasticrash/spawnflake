use crate::name_generator::chain::Chain;
use crate::random_number;

pub fn generate_name(c: &Chain) -> String {
    let token = random_number!(usize)(0, c.tokens.len());
    let mut chain = vec![c.tokens[token]];
    let mut current_chain = c;
    let temp_chain = Chain::new(vec![""]);
    let mut depth = 0;

    loop {
        match *current_chain.rivet {
            Some(data) => {
                let skip = random_number!(usize)(0, 2);
                if skip == 1 {
                    let token = random_number!(usize)(0, data.tokens.len());
                    chain.push(data.tokens[token]);
                }

                current_chain = data.rivet.unwrap_or(&temp_chain);
            }
            None => {
                if depth > 0 {
                    let token = random_number!(usize)(0, current_chain.tokens.len());
                    chain.push(current_chain.tokens[token]);
                }
                break;
            }
        }
        depth += 1;
    }
    chain.join("")
}

#[cfg(test)]
mod tests {
    use crate::name_generator::chain::*;
    use crate::name_generator::name::generate_name;
    #[test]
    fn get_name() {
        let first = Chain::new(vec!["Bg"]);
        assert_eq!(generate_name(&first), "Bg");
    }
}
