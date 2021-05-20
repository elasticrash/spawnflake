use rand::Rng;

pub fn generate_alphas(num_of_chars: i32) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    let mut result = "".to_string();
    for _i in 0..num_of_chars {
        let rng = rand::thread_rng().gen_range(0..25);

        result = format!("{}{}", result, chars.chars().nth(rng).unwrap());
    }
    result
}
