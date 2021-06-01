# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data for relational databases. This is still in its early stages. What it does so far:
* generate data for mysql tables
* supports simple foreign key relationships (non unique foreign keys)
* supports varchar and int types
* varchar types can be configured with specific patterns in the configuration file (config.json) else they will have random strings 
* configuration now supports integers, anything not in the configuration file will generate a random number from 0 to 2147483647

## breaking changes
* O.11 configuration is not backwards compatible with O.10

## Running the example

Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate

## Contributing
You are more than welcome. All you need to do is a pull request
