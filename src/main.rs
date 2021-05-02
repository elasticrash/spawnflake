mod name_generator;
#[macro_use]
mod number_generator;
use crate::name_generator::chain::*;
use crate::name_generator::name::generate_name;

fn main() {
    let mut first = Chain::new(vec!["Jo", "Ni", "Ste", "Da", "Sco", "Ma"]);
    let mut second = Chain::new(vec!["ve", "vi", "pha", "ro", "na", "ri"]);
    let third = Chain::new(vec!["n", "ck", "tt", "d", "than", "na"]);
    second.set(&third);
    first.set(&second);
    println!("{}", generate_name(&first));
    assert_eq!(random_number!(i32)(1, 5) >= 1, true);
}
