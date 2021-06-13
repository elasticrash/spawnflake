use rand::Rng;

/// generates random alpha characters (latin)
/// character pool: abcdefghijklmnopqrstuvwxyz
pub fn generate_alphas(string_type: &str) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    let start_bytes = string_type.find("(").unwrap_or(0);
    let end_bytes = string_type.find(")").unwrap_or(string_type.len());

    let v_size = &string_type[(start_bytes + 1)..end_bytes];
    let mut result = "".to_string();
    for _i in 0..v_size.parse::<i32>().unwrap_or(1) {
        let rng = rand::thread_rng().gen_range(0..25);
        result = format!("{}{}", result, chars.chars().nth(rng).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::string_generator::strings::generate_alphas;

    #[test]
    fn get_random_alpha_string_with_five_chars() {
        let alphas = generate_alphas("varchar(5)");

        assert_eq!(alphas.len(), 5);
        assert_eq!(alphas.chars().all(char::is_alphabetic), true);
    }

    #[test]
    fn get_random_alpha_string_with_255_chars() {
        let alphas = generate_alphas("varchar(255)");

        assert_eq!(alphas.len(), 255);
        assert_eq!(alphas.chars().all(char::is_alphabetic), true);
    }
}
