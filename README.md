# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

carSpawnflake generates random data and/or based on patterns for relational databases. This is still in its early stages. These are the supported features so far (0.1.7):
* db support
    - mysql
    - supports simple foreign key relationships (non unique foreign keys, int)
* supports random data generator for the following types
    * varchar (generates max sized string)
    * int
    * smallint
    * tinyint
    * mediumint
    * bigint
    * decimal 
    * float
    * double
    * bit
    * datetime
* supports pattern based generated random data (config.json)
    * varchar
    * integer

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
