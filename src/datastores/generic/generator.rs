use crate::configuration::config_model::GenericConfiguration;

type BoxFnBool = Box<dyn Fn(&GenericConfiguration, &str) -> bool>;
type BoxFnT<T> = Box<dyn Fn(&GenericConfiguration, &str) -> T>;
type BoxFnString = Box<dyn Fn(&str) -> Option<String>>;

pub struct DataGenerator<T> {
    pub generator_check: BoxFnBool,
    pub generator: BoxFnT<T>,
    pub randomiser: BoxFnString,
    pub default: String,
}
