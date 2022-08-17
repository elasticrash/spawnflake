use crate::{
    date_generator::datetime::{generate_date, generate_datetime, generate_time},
    random_number,
};

use super::const_types::const_types;

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
    } else if ctype.starts_with(const_types::UNSINGED_TINYINT) {
        Some(random_number!(i8)(0, i8::MAX).to_string())
    } else if ctype.starts_with(const_types::SMALLINT) {
        Some(random_number!(i16)(0, i16::MAX).to_string())
    } else if ctype.starts_with(const_types::UNSINGED_SMALLINT) {
        Some(random_number!(u16)(u16::MIN, u16::MAX).to_string())
    } else if ctype.starts_with(const_types::MEDIUMINT) {
        Some(random_number!(i32)(0, 8388607).to_string())
    } else if ctype.starts_with(const_types::INT) {
        Some(random_number!(i32)(0, i32::MAX).to_string())
    } else if ctype.starts_with(const_types::UNSIGNED_INT) {
        Some(random_number!(u32)(u32::MIN, u32::MAX).to_string())
    } else if ctype.starts_with(const_types::BIGINT) {
        Some(random_number!(i64)(0, 2_i64.pow(31) - 1).to_string())
    } else if ctype.starts_with(const_types::UNSINGED_BIGINT) {
        Some(random_number!(i64)(0, 2_i64.pow(31) - 1).to_string())
    } else if ctype.starts_with(const_types::DECIMAL) || ctype.starts_with(const_types::DOUBLE) {
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
                    random_number!(i64)(0, num_a.parse::<i64>().unwrap_or(1)).to_string(),
                    random_number!(i64)(0, num_b.parse::<i64>().unwrap_or(1)).to_string(),
                ));
            } else {
                return Some(format!(
                    "{}",
                    random_number!(i64)(0, num_a.parse::<i64>().unwrap_or(1)).to_string()
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
    } else {
        None
    }
}

pub fn select_enum(ctype: &str) -> Option<String> {
    let start_bytes = ctype.find("(").unwrap_or(0);
    let end_bytes = ctype.find(")").unwrap_or(ctype.len());

    let enum_collection = &ctype[(start_bytes + 1)..end_bytes];
    let enum_values: Vec<&str> = enum_collection.split(",").collect();
    let enum_index = random_number!(usize)(0, enum_values.len());
    Some(enum_values[enum_index].to_string())
}

#[cfg(test)]
mod tests {
    use crate::datastores::my_sql::random_values::{generate_date_time, generate_numeric};

    #[test]
    fn generate_numeric_succesfully() {
        assert!(
            generate_numeric("int")
                .unwrap()
                .parse::<i32>()
                .unwrap_or(-1)
                > 0
        );
    }

    #[test]
    fn generate_year_succesfully() {
        // bits are not supported yet
        assert!(
            generate_date_time("year")
                .unwrap()
                .parse::<i32>()
                .unwrap_or(1)
                > 1969
        );
    }

    #[test]
    fn generate_time_succesfully() {
        // bits are not supported yet
        assert!(generate_date_time("time").unwrap().len() == 8);
    }
}
