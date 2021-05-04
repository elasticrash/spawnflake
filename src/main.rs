mod name_generator;
#[macro_use]
mod number_generator;
use crate::name_generator::name::generate_name;
use crate::name_generator::{
    configuration::{self, config_model::GeneratorConfiguration},
    loader::loader,
};

fn main() {
    let config: GeneratorConfiguration = configuration::reader::read("./config.json").unwrap();
    let chains = loader(&config, "firstname");
    for _i in 0..10 {
        println!("{}", generate_name(&chains));
    }

    let chains = loader(&config, "lastname");
    for _i in 0..10 {
        println!("{}", generate_name(&chains));
    }
}
