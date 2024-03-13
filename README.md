# 🐙❄️ Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake generates random data and/or based on patterns for relational databases.


## what's new
See [CHANGELOG](CHANGELOG.md) for updates and new features.

## known limitations
* When handling cyclic dependencies, keys are not currently updated retrospectively. This feature is planned for a future version
* If a foreign key is unique, fewer records will be inserted into that table. Unique foreign key values are not supported yet.
* In certain cases, number validation in the configuration is not performed, so the db can throw errors for out of range values.

## support 
### Database support 
#### MySQL
* supported data types and generators:

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

## cli 
a pre release version is available/ check [README](cli/readme.md) for usage

## Running the example
Running the example requires running `docker-compose` inside the `test` folder. Make sure you have 
* Docker
* An SQL client/ or a way to access the MySQL cli to view the generated records

To run the example execute

```bash
cargo run --example generate_mysql
```

## Contributing

You are more than welcome. All you need to do is a pull request
