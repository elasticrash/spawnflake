# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data and/or based on patterns for relational databases. This is still in its early stages. These are the supported features so far (0.2.0):
* db support
    - mysql
    - postgres
* supports the following:

| Datastore      | Data type | Random Data Generator | Pattern Based Generator |
| ----------- | ----------- |----------- | ----------- |
| Mysql      | varchar       | ✔️      | ✔️       |
| Mysql   | int        | ✔️      | ✔️       |
| Mysql   | smallint        | ✔️      | ❌       |
| Mysql   | tinyint        | ✔️      | ❌       |
| Mysql   | mediumint        | ✔️      | ❌       |
| Mysql   | bigint        | ✔️      | ❌       |
| Mysql   | decimal        | ✔️      | ❌       |
| Mysql   | float        | ✔️      | ❌       |
| Mysql   | double        | ✔️      | ❌       |
| Mysql   | bit        | ✔️      | ❌       |
| Mysql   | time        | ✔️      | ❌       |
| Mysql   | timestamp        | ✔️      | ❌       |
| Mysql   | date        | ✔️      | ❌       |
| Mysql   | year        | ✔️      | ❌       |
| Mysql   | char        | ✔️      | ❌       |
| Mysql   | binary        | ✔️      | ❌       |
| Mysql   | text        | ✔️      | ❌       |
| Mysql   | blob        | ✔️      | ❌       |
* At the moment it only support tables with numberic ids

## breaking changes
* 0.2.0 completely refactored, configuration is still the same
* 0.1.3 uses the schema from the configuration and does not allow a custom value

## deprecations
* 0.1.0 is going to be yanked

## cli 
under developement

## Running the example
Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate

## Contributing

You are more than welcome. All you need to do is a pull request
