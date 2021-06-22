
pub fn generate_bytes(binary_type: &str) -> i32 {
    if binary_type.starts_with("binary") {
        let start_bytes = binary_type.find("(").unwrap_or(0);
        let end_bytes = binary_type.find(")").unwrap_or(binary_type.len());
        let v_size = &binary_type[(start_bytes + 1)..end_bytes];
        let capacity = v_size.parse::<usize>().unwrap_or(1);
        42
    } else {
        2342523
    }
}
