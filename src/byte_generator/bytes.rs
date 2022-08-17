use crate::{
    datastores::{
        my_sql::const_types::const_types as sqlt, postgres::const_types::const_types as psqlt,
    },
    random_number,
};

pub fn generate_bytes(binary_type: &str) -> String {
    let mut result = "".to_string();
    if binary_type.starts_with(sqlt::BINARY)
        || binary_type.starts_with(sqlt::VARBINARY)
        || binary_type.starts_with(psqlt::BYTE)
    {
        let start_bytes = binary_type.find("(").unwrap_or(0);
        let end_bytes = binary_type.find(")").unwrap_or(binary_type.len());
        let v_size = &binary_type[(start_bytes + 1)..end_bytes];
        let capacity = v_size.parse::<usize>().unwrap_or(1);
        for _i in 0..capacity {
            result = format!("{}{:X}", result, random_number!(i32)(0, 16));
            if result.len() % 2 != 0 {
                result = format!("0{}", result);
            }
        }
    } else if binary_type.eq(sqlt::BLOB) || binary_type.eq(sqlt::LONG_BLOB) {
        for _i in 0..50 {
            result = format!("{}{:X}", result, random_number!(i32)(0, 16));
            if result.len() % 2 != 0 {
                result = format!("0{}", result);
            }
        }
    } else {
        panic!("unknown type");
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::byte_generator::bytes::generate_bytes;

    #[test]
    fn get_random_hex_string_with_three_chars_successfully() {
        let hex = generate_bytes("binary(3)");

        assert_eq!(hex.chars().all(|a| a.is_ascii_hexdigit()), true);
    }

    #[test]
    fn get_random_blob_string_successfully() {
        let hex = generate_bytes("blob");

        assert_eq!(hex.chars().all(|a| a.is_ascii_hexdigit()), true);
    }

    #[test]
    #[should_panic]
    fn unknown_type_panics() {
        generate_bytes("random");
    }
}
