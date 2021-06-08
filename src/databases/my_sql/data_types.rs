use crate::random_number;

pub fn check_if_numeric(ctype: &str) -> bool {
    let accepted_values = vec![
        "int",
        "smallint",
        "tinyint",
        "mediumint",
        "bigint",
        "decimal",
        "float",
        "double",
    ];

    accepted_values.into_iter().any(|x| ctype.starts_with(x))
}

pub fn generate_numeric(ctype: &str) -> String {
    if ctype.starts_with("tinyint") {
        random_number!(i8)(0, i8::MAX).to_string()
    } else if ctype.starts_with("smallint") {
        random_number!(i16)(0, i16::MAX).to_string()
    } else if ctype.starts_with("mediumint") {
        random_number!(i32)(0, 8388607).to_string()
    } else if ctype.starts_with("int") {
        random_number!(i32)(0, i32::MAX).to_string()
    } else if ctype.starts_with("bigint") {
        random_number!(i128)(0, 2_i128.pow(63) - 1).to_string()
    } else if ctype.starts_with("decimal") {
        random_number!(f32)(0., 10.).to_string()
    } else if ctype.starts_with("float") {
        random_number!(f32)(0., f32::MAX).to_string()
    } else if ctype.starts_with("double") {
        random_number!(f32)(0., f32::MAX).to_string()
    } else {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::databases::my_sql::data_types::check_if_numeric;

    #[test]
    fn check_if_int_is_numeric() {
        assert_eq!(check_if_numeric("int"), true);
    }

    #[test]
    fn check_if_bit1_is_numeric() {
        // bits are not supported yet
        assert_eq!(check_if_numeric("bit(1)"), false);
    }
}
