use rand::Rng;

/// generates random alpha characters (latin)
/// character pool: abcdefghijklmnopqrstuvwxyz
pub fn generate_alphas(num_of_chars: i32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    let mut result = "".to_string();
    for _i in 0..num_of_chars {
        let rng = rand::thread_rng().gen_range(0..25);

        result = format!("{}{}", result, chars.chars().nth(rng).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::string_generator::strings::generate_alphas;

    #[test]
    fn get_random_alpha_string() {
        let length = 5;
        let alphas= generate_alphas(length);

        assert_eq!(alphas.len(), 5);
        assert_eq!(alphas.chars().all(char::is_alphabetic), true);
    }
}