# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data and/or based on patterns for relational databases. This is still in its early stages. These are the supported features so far (0.2.0):
* db support
    - mysql
    - postgres (only tested on public schema)
* supports the following types and functionality:

| Datastore      | Data type | Random Data Generator | Pattern Based Generator |
| ----------- | ----------- |----------- | ----------- |
| Mysql/Postgres+      | varchar       | ✔️      | ✔️       |
| Mysql/Postgres   | int        | ✔️      | ✔️       |
| Mysql/Postgres   | smallint        | ✔️      | ❌       |
| Mysql   | tinyint        | ✔️      | ❌       |
| Mysql   | mediumint        | ✔️      | ❌       |
| Mysql/Postgres   | bigint        | ✔️      | ❌       |
| Mysql/Postgres+   | decimal        | ✔️      | ❌       |
| Mysql   | float        | ✔️      | ❌       |
| Mysql/Postgres   | double        | ✔️      | ❌       |
| Mysql   | bit        | ✔️      | ❌       |
| Mysql/Postgres   | time        | ✔️      | ❌       |
| Mysql/Postgres   | timestamp        | ✔️      | ❌       |
| Mysql/Postgres   | date        | ✔️      | ❌       |
| Mysql   | year        | ✔️      | ❌       |
| Mysql/Postgres   | char        | ✔️      | ❌       |
| Mysql   | binary        | ✔️      | ❌       |
| Mysql/Postgres   | text        | ✔️      | ❌       |
| Mysql   | blob        | ✔️      | ❌       |
| Postgres   | real        | ✔️      | ❌       |
| Postgres+  | interval        | ✔️      | ❌       |
| Postgres  | byte        | ✔️      | ❌       |
| Postgres  | boolean        | ✔️      | ❌       |

* At the moment it only support tables with numberic ids

(+) works but not necessary with expected outcome
## breaking changes
* 0.2.0 was completely refactored, configuration is stays the same
* 0.1.3 uses the schema from the configuration and does not allow a custom value

## deprecations
* 0.1.0 is going to be yanked

## cli 
a pre release version is available/ check cli/readme.md for usage

## Running the example
Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate

## Contributing

You are more than welcome. All you need to do is a pull request
