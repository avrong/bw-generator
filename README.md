# BuzzWord Generator

A buzzword statements generator server in Rust.

Inspired by [sameerkumar18/corporate-bs-generator-api](https://github.com/sameerkumar18/corporate-bs-generator-api).

## Try It!

#### ```GET https://bw-generator.herokuapp.com/```
```json
{"phrase": "Quickly whiteboard inexpensive innovation"}
```

## Usage

```
bw_generator 1.0
Aleksei T. <avrong@avrong.me>
BuzzWord Generator Server

USAGE:
    bw_generator [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dictionary <dictionary>    Buzzword dictionary to use [default: dictionary.toml]
    -h, --host <host>                Address to bind to [default: 127.0.0.1]
    -p, --port <port>                Port to bind to [default: 3000]
```

#### GET ```/```
> Returns a random generated buzzword statement.
