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
        let start_bytes = ctype.find("(").unwrap_or(0);
        let end_bytes = ctype.find(")").unwrap_or(ctype.len());

        let d_size = ctype[(start_bytes + 1)..end_bytes].split(",");
        let md_size = d_size.collect::<Vec<&str>>();
        if md_size.len() == 2 {
            let a = md_size[0].parse::<i32>().unwrap_or(1);
            let b = md_size[1].parse::<i32>().unwrap_or(1);

            let mut num_a = "".to_string();
            for _i in 0..a {
                num_a = format!("{}{}", num_a, 9);
            }

            if b > 0 {
                let mut num_b = "".to_string();
                for _i in 0..b {
                    num_b = format!("{}{}", num_b, 9);
                }
                num_a.truncate(i32::abs(num_a.len() as i32 - num_b.len() as i32) as usize);
                return format!(
                    "{}.{}",
                    random_number!(i128)(0, num_a.parse::<i128>().unwrap_or(1)).to_string(),
                    random_number!(i128)(0, num_b.parse::<i128>().unwrap_or(1)).to_string(),
                );
            } else {
                return format!("{}", random_number!(i128)(0, num_a.parse::<i128>().unwrap_or(1)).to_string());
            }
        }
        return format!(
            "{}.{}",
            random_number!(i32)(1, 10).to_string(),
            random_number!(i32)(1, 10).to_string(),
        );
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
