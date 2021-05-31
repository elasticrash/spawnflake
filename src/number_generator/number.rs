use crate::configuration::config_model::GenericConfiguration;

/// macro that generates a random number regardless the type
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

/// generates a random number (int) based on configuration rules
pub fn generate_int_number(config: &GenericConfiguration, name: &str) -> i32 {
    let pattern = config
        .clone()
        .types
        .integer
        .into_iter()
        .find(|x| x.name == name)
        .unwrap();

    random_number!(i32)(pattern.rules[0], pattern.rules[1])
}

/// Checks if a integer generator exists
pub fn int_generator_exists(config: &GenericConfiguration, name: &str) -> bool {
    let name_generator = &config.types.integer.iter().find(|x| x.name == name);

    match name_generator {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod tests {
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
