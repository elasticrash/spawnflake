use rand::Rng;

use crate::datastores::my_sql::const_types::db_types;

/// generates random alpha characters (latin)
/// character pool: abcdefghijklmnopqrstuvwxyz
pub fn generate_alphas(string_type: &str) -> Option<String> {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    if string_type.eq(db_types::TEXT)
        || string_type.eq(db_types::LONG_TEXT)
        || string_type.eq(db_types::MEDIUM_TEXT)
    {
        let mut result = "".to_string();
        for _i in 0..255 {
            let rng = rand::thread_rng().gen_range(0..25);
            result = format!("{}{}", result, chars.chars().nth(rng).unwrap());
        }
        Some(result)
    } else {
        let start_bytes = string_type.find('(').unwrap_or(0);
        let end_bytes = string_type.find(')').unwrap_or(string_type.len());

        let v_size = &string_type[(start_bytes + 1)..end_bytes];
        let mut result = "".to_string();
        for _i in 0..v_size.parse::<i32>().unwrap_or(1) {
            let rng = rand::thread_rng().gen_range(0..25);
            result = format!("{}{}", result, chars.chars().nth(rng).unwrap());
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::string_generator::strings::generate_alphas;

    #[test]
    fn get_random_alpha_string_with_five_chars() {
        let alphas = generate_alphas("varchar(5)").expect("string");

        assert_eq!(alphas.len(), 5);
        assert_eq!(alphas.chars().all(char::is_alphabetic), true);
    }

    #[test]
    fn get_random_alpha_string_with_255_chars() {
        let alphas = generate_alphas("varchar(255)").expect("string");

        assert_eq!(alphas.len(), 255);
        assert_eq!(alphas.chars().all(char::is_alphabetic), true);
    }
}
