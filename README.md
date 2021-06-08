# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data for relational databases. This is still in its early stages. What it does so far:
* generate data for mysql tables
* supports simple foreign key relationships (non unique foreign keys)
* supports random data for varchar,
* supports random data for most numberic types
 ```    
    "int",
    "smallint",
    "tinyint",
    "mediumint",
    "bigint",
    "decimal",
    "float",
    "double",
```
* supports random data for datetime type
* configured values for varchars based on specific patterns in the configuration file (config.json)
* configuration values for integers

## breaking changes
* 0.1.1 configuration is not backwards compatible with 0.1.0
* 0.1.3 uses the schema from the configuration and does not allow a custom value

## Running the example
Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate

## Contributing
You are more than welcome. All you need to do is a pull request
