use crate::random_number;

pub fn generate_bytes(binary_type: &str) -> String {
    let mut result = "".to_string();
    if binary_type.starts_with("binary") {
        let start_bytes = binary_type.find("(").unwrap_or(0);
        let end_bytes = binary_type.find(")").unwrap_or(binary_type.len());
        let v_size = &binary_type[(start_bytes + 1)..end_bytes];
        let capacity = v_size.parse::<usize>().unwrap_or(1);
        for _i in 0..capacity {
            result = format!("{}{:X}", result, random_number!(i32)(0, 16));
        }
    } else {
        for _i in 0..50 {
            result = format!("{}{:X}", result, random_number!(i32)(0, 16))
        }
    }
    result
}
