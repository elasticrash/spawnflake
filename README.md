# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data and/or based on patterns for relational databases.


## what's new
See [CHANGELOG](CHANGELOG.md)

## known limitations
* In the cases of handling cyclic dependencies I am not currently updating the keys retrospectively. This is planned for a future version
* If a foreign key is unique, less records will be inserted on that table (I do not support unique foreign key values yet)
* In certain cases number validation in the configuration is not performed, so the db can throw errors for out of range values.

## support 
* db support
    - mysql
* supports the following types and generators:

| Datastore   | Data type                | Random    | Pattern      | 
| ----------- | -----------              |-----------| -----------  | 
| Mysql       | varchar                  | ✔️         | ✔️            |
| Mysql       | int                      | ✔️         | ✔️            |
| Mysql       | unsigned int             | ✔️         | ✔️            |
| Mysql       | smallint                 | ✔️         | ✔️            |
| Mysql       | unsigned smallint        | ✔️         | ✔️            |
| Mysql       | tinyint/unsigned tinyint | ✔️         | ✔️            |
| Mysql       | mediumint                | ✔️         | ✔️            |
| Mysql       | bigint                   | ✔️         | ✔️            |
| Mysql       | unsigned bigint          | ✔️         | ✔️            |
| Mysql       | decimal                  | ✔️         | ✔️            |
| Mysql       | float                    | ✔️         | ✔️            |
| Mysql       | double                   | ✔️         | ✔️            |
| Mysql       | bit                      | ✔️         | ❌           |
| Mysql       | time                     | ✔️         | ❌           |
| Mysql       | timestamp                | ✔️         | ❌           |
| Mysql       | date                     | ✔️         | ❌           |
| Mysql       | year                     | ✔️         | ❌           |
| Mysql       | char                     | ✔️         | ❌           |
| Mysql       | binary                   | ✔️         | ❌           |
| Mysql       | text                     | ✔️         | ❌           |
| Mysql       | longtext                 | ✔️         | ❌           |
| Mysql       | blob/longblob            | ✔️         | ❌           |
| Mysql       | enum                     | ❌        | ✔️            |

(+) works but not necessary with expected outcome

## cli 
a pre release version is available/ check cli/readme.md for usage

## Running the example
Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example generate_mysql

## Contributing

You are more than welcome. All you need to do is a pull request
