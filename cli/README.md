# 🐙❄️ Spawnflake(s)

This is the cli tool the loyal companion of the spanwflake libary

## USAGE

```
spawnflakes.exe [OPTIONS]

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
spawnflakes.exe -s 4 -c ..\config.json
```