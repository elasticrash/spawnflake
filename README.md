# 🐙❄️ Spawnflake 

Spawnflake generates random data for relational databases. This is still in its early stages. What it does so far:
* generate data for mysql tables with no relations (no foreign keys)
* supports varchar and int types
* varchar types can be configured with specific patterns in the configuration file (config.json) else they will have random strings 
* configuration now supports integers, anything not in the configuration file will generate a random number from 0 to 2147483647

## breaking changes
* O.11 configuration is not backwards compatible with O.10

## Running the example

Running the example requires to run docker-compose inside the test folder. Obviously you need
* docker
* an sql client/ or a way to access the mysql cli (so as to see the generated records)
* cargo run --example single_table

## Contributing
You are more than welcome