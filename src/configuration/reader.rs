extern crate serde;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::config_model::GenericConfiguration;

/**
 * ## Reads configuration from provided file
 * If file does not exist or we cannot read the content of the file,
 * we use the default values
 */
pub fn read(filename: &str) -> serde_json::Result<GenericConfiguration> {
    let mut buffer = String::new();
    match File::open(filename) {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
            let config = serde_json::from_str(&buffer);
            println!(
                "* Reading Configuration from {:?}",
                Path::new(filename).file_name()
            );
            config
        }
        Err(why) => {
            println!("{why}");
            panic!()
        }
    }
}
