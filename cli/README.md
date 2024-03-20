# ![a pixel fish](./assets/logo.png "fish") spawnflake-cli

This is the cli tool the loyal companion of the spanwflake libary
It should work on both windows and linux (mostly tested in linux)

## Usage

```
spawnflake-cli[.exe] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <configuration>               sets a custom configuration name and path relative to this location
    -d <datastore>                   Sets the type of the datastore to generate values. By default executes the entire
                                     configuration
    -s, --spawn-size <spawn-size>    Set the number of rows per table to be generated
```
example
```
spawnflake-cli[.exe] -s 4 -c ..\config.json
```

## Configuration

* Refer to spawnflake's [README](../README.md) or
* Check out the sample file provided: [configuration](../config.json)

## Installing
* You can build it from source by cloning this repo and running `cargo build --release`
* Or you can install it directly from crates.io Just do `cargo install spawnflake-cli`. In this case, though, make sure that `~/.cargo/bin/` is in your PATH variable
