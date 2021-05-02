fn main() {
    let mut first = Chain::new(vec!["Jo", "Ni", "Ste", "Da", "Sco", "Ma"]);
    let mut second = Chain::new(vec!["ve", "vi", "pha", "ro", "na", "ri"]);
    let third = Chain::new(vec!["n", "ck", "tt", "d", "than", "na"]);
    second.set(&third);
    first.set(&second);
    println!("{}", straight_chain(&first));
}

#[macro_export]
macro_rules! random_number {
    ($calc:ty) => {{
        use rand::Rng;
        pub fn rnd(a: $calc, b: $calc) -> $calc {
            let rng = rand::thread_rng().gen_range(a..b);

            rng
        }
        rnd
    }};
}

#[derive(Debug)]
struct Chain<'a> {
    pub tokens: Vec<&'a str>,
    pub rivet: Box<Option<&'a Chain<'a>>>,
}

impl<'a> Chain<'a> {
    fn new(tokens: std::vec::Vec<&'static str>) -> Chain<'static> {
        Chain {
            tokens: tokens,
            rivet: Box::new(None),
        }
    }
    fn set(&mut self, r: &'a Chain) {
        self.rivet = Box::new(Some(r));
    }
}

fn straight_chain(c: &Chain) -> String {
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
    use crate::random_number;
    use crate::straight_chain;
    use crate::Chain;

    #[test]
    fn get_name() {
        let first = Chain::new(vec!["Bg"]);
        assert_eq!(straight_chain(&first), "Bg");
    }

    #[test]
    fn get_random_i32_between_two_values() {
        assert_eq!(random_number!(i32)(1, 5) >= 1, true);
    }

    #[test]
    fn get_random_i64_between_two_values() {
        assert_eq!(
            random_number!(i64)(100000000000, 5100000000000) >= 100000000000,
            true
        );
    }

    #[test]
    fn get_random_f32_between_two_values() {
        assert_eq!(random_number!(f32)(1.1, 5.3) >= 1.1, true);
    }

    #[test]
    fn get_random_f64_between_two_values() {
        assert_eq!(
            random_number!(f64)(
                1000000000000000000000000000000000000000.1,
                5000000000000000000000000000000000000000.3
            ) >= 1000000000000000000000000000000000000000.1,
            true
        );
    }
}
