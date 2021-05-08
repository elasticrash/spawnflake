mod name_generator;
#[macro_use]
mod number_generator;
mod configuration;
use crate::configuration::config_model::GeneratorConfiguration;
use crate::name_generator::loader::loader;
use crate::name_generator::name::generate_name;

fn main() {
    let config: GeneratorConfiguration = configuration::reader::read("./config.json").unwrap();
    let firstname = loader(&config, "firstname");
    let lastname = loader(&config, "lastname");

    for _i in 0..10 {
        println!("{} {}", generate_name(&firstname), generate_name(&lastname));
    }
}
