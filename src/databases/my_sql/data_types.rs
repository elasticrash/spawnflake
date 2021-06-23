use crate::{
    databases::generic::const_types::const_types,
    date_generator::datetime::{generate_date, generate_datetime, generate_time},
    random_number,
};

pub fn check_if_numeric(ctype: &str) -> bool {
    let accepted_values = vec![
        const_types::INT,
        const_types::SMALLINT,
        const_types::TINYINT,
        const_types::MEDIUMINT,
        const_types::BIGINT,
        const_types::DECIMAL,
        const_types::FLOAT,
        const_types::DOUBLE,
    ];

    accepted_values.into_iter().any(|x| ctype.starts_with(x))
}

pub fn check_if_date_time(ctype: &str) -> bool {
    let accepted_values = vec![
        const_types::DATETIME,
        const_types::DATE,
        const_types::TIMESTAMP,
        const_types::TIME,
        const_types::YEAR,
    ];

    accepted_values.into_iter().any(|x| ctype.starts_with(x))
}

pub fn check_if_string(ctype: &str) -> bool {
    let accepted_values = vec![const_types::VARCHAR, const_types::CHAR, const_types::TEXT];

    accepted_values.into_iter().any(|x| ctype.starts_with(x))
}

pub fn check_if_binary(ctype: &str) -> bool {
    let accepted_values = vec![const_types::BINARY, const_types::BLOB];

    accepted_values.into_iter().any(|x| ctype.starts_with(x))
}

pub fn generate_date_time(ctype: &str) -> Option<String> {
    if ctype.starts_with(const_types::DATETIME) {
        Some(generate_datetime().to_string())
    } else if ctype.starts_with(const_types::DATE) {
        Some(generate_date().to_string())
    } else if ctype.starts_with(const_types::TIMESTAMP) {
        Some(generate_datetime().to_string())
    } else if ctype.starts_with(const_types::TIME) {
        Some(generate_time().to_string())
    } else if ctype.starts_with(const_types::YEAR) {
        Some(random_number!(i32)(1970, 2021).to_string())
    } else {
        None
    }
}

pub fn generate_numeric(ctype: &str) -> Option<String> {
    if ctype.starts_with(const_types::TINYINT) {
        Some(random_number!(i8)(0, i8::MAX).to_string())
    } else if ctype.starts_with(const_types::SMALLINT) {
        Some(random_number!(i16)(0, i16::MAX).to_string())
    } else if ctype.starts_with(const_types::MEDIUMINT) {
        Some(random_number!(i32)(0, 8388607).to_string())
    } else if ctype.starts_with(const_types::INT) {
        Some(random_number!(i32)(0, i32::MAX).to_string())
    } else if ctype.starts_with(const_types::BIGINT) {
        Some(random_number!(i128)(0, 2_i128.pow(63) - 1).to_string())
    } else if ctype.starts_with(const_types::DECIMAL) {
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
                return Some(format!(
                    "{}.{}", //TODO: prefix zeros
                    random_number!(i128)(0, num_a.parse::<i128>().unwrap_or(1)).to_string(),
                    random_number!(i128)(0, num_b.parse::<i128>().unwrap_or(1)).to_string(),
                ));
            } else {
                return Some(format!(
                    "{}",
                    random_number!(i128)(0, num_a.parse::<i128>().unwrap_or(1)).to_string()
                ));
            }
        }
        return Some(format!(
            "{}.{}",
            random_number!(i32)(1, 10).to_string(),
            random_number!(i32)(1, 10).to_string(),
        ));
    } else if ctype.starts_with(const_types::FLOAT) {
        Some(random_number!(f32)(0., f32::MAX).to_string())
    } else if ctype.starts_with(const_types::DOUBLE) {
        Some(random_number!(f32)(0., f32::MAX).to_string())
    } else {
        None
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
