#[macro_use]
extern crate yasec_derive;
extern crate yasec;

use std::env;
use std::error::Error as _;
use yasec::Yasec;

#[derive(Yasec)]
pub struct DBConfig {
    #[yasec(from = "DB_HOST")]
    pub host: String,

    #[yasec(from = "DB_PORT")]
    pub port: u16,
}

#[derive(Yasec)]
pub struct Config {
    pub db: DBConfig,
}

#[derive(Yasec)]
pub struct ConfigDouble {
    pub db1: DBConfig,
    pub db2: DBConfig,
}

fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_neasting() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init().unwrap();
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, 5432u16);
}

#[test]
fn test_neasting_error() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init().err().unwrap();

    // let expected_err = Error::EnvVarMissing { name: "DB_PORT".to_owned() };
    assert_eq!(true, err.source().unwrap().is::<env::VarError>());
}

#[test]
fn test_duplicated_are_allowed() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = ConfigDouble::init().unwrap();
    assert_eq!(config.db1.host, "localhost");
    assert_eq!(config.db1.port, 5432u16);
    assert_eq!(config.db2.host, "localhost");
    assert_eq!(config.db2.port, 5432u16);
}
