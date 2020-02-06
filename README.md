# Yasec

[![Build Status](https://travis-ci.org/ANtlord/yasec.svg?branch=master)](https://travis-ci.org/ANtlord/yasec)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Build a config structure from environment variables in Rust without boilerplate.

## Features

* Nested configuration structure.
* Inferring of an environment variable name. If a configuration field has name `password` then it gets value from environment variable `PASSWORD`.
If a configuration field is inside another structure and it has path `db.password` it gets its value from variable `DB_PASSWORD`.
* Custom types.
* Option type is optional.
* Prefix of variables.

## Macro attributes

* `from` - name of an environment variable which provides a field value. Name of the field and name of the parent structures are ignored.
* `default` - default value of a field if an environment variable doesn't exist. If the environment variable exist but has invalid value an error returns.

## Usage

You can achieve this with the following code without boilerplate:

```rust
#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use std::error::Error as StdError;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct DB {
    pub host: String,
    pub port: u16,
}

#[derive(Envconfig)]
pub struct Vendor {
    #[envconfig(from = "API_KEY")]
    pub key: String,
    #[envconfig(from = "API_SECRET")]
    pub secret: String,
}

#[derive(Envconfig)]
pub struct Config {
    db: DB,
    vendor: Vendor,
    #[envconfig(default = 8080)]
    listen_port: u16,
    callback_url: Option<String>,
    mode: Mode,
}

pub enum Mode {
    Client,
    Server,
}

impl Envconfig for Mode {
    fn parse(s: &str) -> Result<Self, Box<dyn StdError>> {
        match s {
            "CLIENT" => Ok(Self::Client),
            "SERVER" => Ok(Self::Server),
            _ => Err(envconfig::ParseError::new(s).into()),
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
