# BuzzWord Generator

A buzzword statements generator server in Rust.

```
{"phrase": "Quickly whiteboard inexpensive innovation"}
```

Inspired by [sameerkumar18/corporate-bs-generator-api](https://github.com/sameerkumar18/corporate-bs-generator-api).

## Endpoints

#### ```GET /```
```Returns a random generated buzzword statement.```

## Usage

```
bw-generator [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dict <FILE>       The buzzword dictionary to use (default: dictionary.toml)
    -h, --host <TEXT>       The interface to bind to (default: 127.0.0.1)
    -p, --port <INTEGER>    The port to bind to (default: 3000)
```
