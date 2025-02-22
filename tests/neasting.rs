use std::env;
use yasec::Yasec;

#[derive(Yasec)]
pub struct DBConfig {
    #[yasec(env = "DB_HOST")]
    pub host: String,

    #[yasec(env = "DB_PORT")]
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
fn test_nesting() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init().unwrap();
    assert_eq!(config.db.host, "localhost");
    assert_eq!(config.db.port, 5432u16);
}

#[test]
fn test_nesting_error() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init().err().unwrap();

    assert_eq!(err, yasec::YasecError::EmptyVar("DB_PORT".to_owned()));
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
