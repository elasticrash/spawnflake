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
