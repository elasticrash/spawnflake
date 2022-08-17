use crate::{
    date_generator::datetime::{generate_date, generate_datetime, generate_time},
    random_number,
};

use super::const_types::const_types;

pub fn generate_numeric(ctype: &str) -> Option<String> {
    if ctype.starts_with(const_types::INT) {
        Some(random_number!(i8)(0, i8::MAX).to_string())
    } else if ctype.starts_with(const_types::REAL) {
        Some(random_number!(i8)(0, i8::MAX).to_string())
    } else if ctype.starts_with(const_types::BIGINT) {
        Some(random_number!(i16)(0, i16::MAX).to_string())
    } else if ctype.starts_with(const_types::DECIMAL) {
        Some(random_number!(i32)(0, 8388607).to_string())
    } else if ctype.starts_with(const_types::DOUBLE) {
        Some(random_number!(f32)(0., f32::MAX).to_string())
    } else {
        None
    }
}

pub fn generate_date_time(ctype: &str) -> Option<String> {
    if ctype.starts_with(const_types::DATE) {
        Some(generate_datetime().to_string())
    } else if ctype.starts_with(const_types::TIMESTAMP) {
        Some(generate_date().to_string())
    } else if ctype.starts_with(const_types::TIME) {
        Some(generate_datetime().to_string())
    } else if ctype.starts_with(const_types::INTERVAL) {
        Some(generate_time().to_string())
    } else {
        None
    }
}
