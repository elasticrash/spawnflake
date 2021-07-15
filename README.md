# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data and/or based on patterns for relational databases. This is still in its early stages.

## whats new (v0.2.1)
* The libray now builds a depedency tree of the foreign keys so it will know in which order it needs to generate the tables (mysql only, experimental) 
* Supports more types 
* Supports column names with reserve names
## support 
* db support
    - mysql
    - postgres (only tested on public schema)
* supports the following types and functionality:

| Datastore      | Data type | Random Data Generator | Pattern Based Generator |
| ----------- | ----------- |----------- | ----------- |
| Mysql/Postgres+      | varchar       | ✔️      | ✔️       |
| Mysql/Postgres   | int        | ✔️      | ✔️       |
| Mysql   | unsigned int        | ✔️      | ❌       |
| Mysql/Postgres   | smallint        | ✔️      | ❌       |
| Mysql   | unsigned smallint        | ✔️      | ❌       |
| Mysql   | tinyint/unsigned tinyint        | ✔️      | ❌       |
| Mysql   | mediumint        | ✔️      | ❌       |
| Mysql/Postgres   | bigint       | ✔️      | ❌       |
| Mysql   | unsigned bigint        | ✔️      | ❌       |
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
| Mysql   | longtext        | ✔️      | ❌       |
| Mysql   | blob/longblob        | ✔️      | ❌       |
| Mysql   | enum        | ❌      | ✔️       |
| Postgres   | real        | ✔️      | ❌       |
| Postgres  | interval        | ✔️      | ❌       |
| Postgres  | byte        | ✔️      | ❌       |
| Postgres  | boolean        | ✔️      | ❌       |

* At the moment it only support tables with numberic ids

(+) works but not necessary with expected outcome
## breaking changes
* 0.2.0 was completely refactored, configuration is stays the same

## deprecations
* 0.1.0 to 0.1.3 is going to be yanked

## cli 
a pre release version is available/ check cli/readme.md for usage

## Running the example
Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate_mysql

## Contributing

You are more than welcome. All you need to do is a pull request
