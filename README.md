# ![a pixel fish](./assets/logo.png "fish") Spawnflake [![Rust](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml/badge.svg)](https://github.com/elasticrash/spawnflake/actions/workflows/rust.yml)

Spawnflake is a schema agnostic, random and/or patterns based data generator, for MySQL databases.

## what's new
See [CHANGELOG](CHANGELOG.md) for updates and new features.

## usage
This library works by providing a configuration file (config.json). The configuration is divided into two sections.
### connection properties (mandatory)
```json 
 "mysql_configuration": {
        "address": "localhost",
        "port": 3306,
        "user": "local",
        "password": "password",
        "schema": "test"
    }
```
### type patterns (optional)
```json
 "types": {
        "string": [
            {
                "name": "column_name",
                "rules": []
            }
        ],
        "integer": [
            {
                "name": "column_name",
                "rules":[]
            }
        ],
        "float": [
            {
                "name": "column_name",
                "rules":[]
            }
        ]
    }
```
### Rules
* Rules for numeric types and dates just desired ranges (from, to). **Important!!** the small values need to be first in the lists. 
* Rules for strings are combinatinatory, you provide collections that are mixed and matched.
i.e.

```json
   "rules": [
                    [
                        "Jo",
                        "Ni",
                        "Ste",
                        "Da",
                        "Sco",
                        "Ma"
                    ],
                    [
                        "ve",
                        "vi",
                        "pha",
                        "ro",
                        "na",
                        "ri"
                    ],
                    [
                        "n",
                        "ck",
                        "tt",
                        "d",
                        "than",
                        "na"
                    ]
                ]
```
This could create real names like `David` but also complete random combinations like `Jophack`

## known limitations
* When handling cyclic dependencies, keys are not currently updated retrospectively. This feature is planned for a future version
* If a foreign key is unique, fewer records will be inserted into that table. Unique foreign key values are not supported yet.
* In certain cases, number validation in the configuration is not performed, records assosiated with problematic talbes can be 
  * skipped
  * panic
  * use default values

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
| Mysql       | timestamp                | ✔️         | ✔️            |
| Mysql       | date                     | ✔️         | ❌           |
| Mysql       | year                     | ✔️         | ❌           |
| Mysql       | char                     | ✔️         | ❌           |
| Mysql       | binary                   | ✔️         | ❌           |
| Mysql       | text                     | ✔️         | ❌           |
| Mysql       | longtext                 | ✔️         | ❌           |
| Mysql       | blob/longblob            | ✔️         | ❌           |
| Mysql       | enum                     | ❌        | ✔️            |

## CLI
The main project in this repository is a library, but there is also a [CLI version](https://crates.io/crates/spawnflake-cli) available published in crates.io.
The code for the CLI is available at `./cli`. Check [README](cli/readme.md) for usage and more information.

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
