# Alfatm Yasec

This is fork of https://github.com/ANtlord/yasec

## Added features:

- add "usage" command to print available environment variables
- support bool type
- support humantime::Duration type
- support bytesize::ByteSize type
- support string vector and int vector type
- keyword "default" always contains a string representation of environment variable value, that parsed in runtime
- Context is not a generic type anymore
- rename keyword "from" to "env"
- change edition to 2021

# Yasec

[![Build Status](https://travis-ci.org/ANtlord/yasec.svg?branch=master)](https://travis-ci.org/ANtlord/yasec)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Yet another stupid environment config (YASEC) creates settings from environment variables. (Envconig-rs fork)

## Features

- Nested configuration structure.
- Inferring of an environment variable name. If a configuration field has name `password` then it gets value from environment variable `PASSWORD`.
  If a configuration field is inside another structure and it has path `db.password` it gets its value from variable `DB_PASSWORD`.
- Custom types.
- Option type is optional.
- Prefix of variables.

I implemented everything what I require when I develop an application. Feel free to open an issue of a feature you miss as well as a pull request.

## Macro attributes

- `from` - name of an environment variable which provides a field value. Name of the field and name of the parent structures are ignored.
- `default` - default value of a field if an environment variable doesn't exist. If the environment variable exist but has invalid value an error returns.

## Usage

You can achieve this with the following code without boilerplate:

```rust
#[macro_use]
extern crate yasec_derive;
extern crate yasec;

use std::error::Error as StdError;
use yasec::Yasec;

#[derive(Yasec)]
pub struct DB {
    pub host: String,
    pub port: u16,
}

#[derive(Yasec)]
pub struct Vendor {
    #[yasec(env = "API_KEY")]
    pub key: String,
    #[yasec(env = "API_SECRET")]
    pub secret: String,
}

#[derive(Yasec)]
pub struct Config {
    db: DB,
    vendor: Vendor,
    #[yasec(default = "8080")]
    listen_port: u16,
    callback_url: Option<String>,
    mode: Mode,
}

pub enum Mode {
    Client,
    Server,
}

impl Yasec for Mode {
    fn parse(s: &str) -> Result<Self, Box<dyn StdError>> {
        match s {
            "CLIENT" => Ok(Self::Client),
            "SERVER" => Ok(Self::Server),
            _ => Err(yasec::ParseError::new(s).into()),
        }
    }
}

fn main() {
    // Assuming the following environment variables are set
    std::env::set_var("DB_HOST", "127.0.0.1");
    std::env::set_var("DB_PORT", "5432");
    std::env::set_var("API_KEY", "0912xn819b8s1029s");
    std::env::set_var("API_SECRET", "zyYWn5pPtLcDSaFWQEu0nf1cf0eYNN8j");
    std::env::set_var("MODE", "SERVER");
    std::env::remove_var("LISTEN_PORT");
    std::env::remove_var("CALLBACK_URL");

    // Initialize config from environment variables or terminate the process.
    let config = Config::init().unwrap();

    assert_eq!(config.db.host, "127.0.0.1");
    assert_eq!(config.db.port, 5432);
    assert_eq!(config.vendor.key, "0912xn819b8s1029s");
    assert_eq!(config.vendor.secret, "zyYWn5pPtLcDSaFWQEu0nf1cf0eYNN8j");
    assert_eq!(config.listen_port, 8080);
    assert_eq!(config.callback_url, None);
    match config.mode {
        Mode::Server => (),
        _ => panic!("Unexpected value of Mode"),
    }
}
```

## Running tests

Tests do some manipulation with environment variables, so to
prevent flaky tests they have to be executed in a single thread:

```
cargo test -- --test-threads=1
```

## License

Licensed under [MIT](LICENSE)
