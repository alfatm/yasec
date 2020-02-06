#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use std::env;
use std::error::Error as _;
use std::num::ParseIntError;

use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT")]
    pub db_port: u16,
}

fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_inits_config_from_env_variables() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init().unwrap();
    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5432u16);
}

#[test]
fn test_checks_presence_of_env_vars() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init().err().unwrap();
    // let expected_err = Error::EnvVarMissing { name: "DB_PORT".to_owned() };
    // assert_eq!(err, expected_err);
    assert_eq!(true, err.source().unwrap().is::<env::VarError>());
}

#[test]
fn test_fails_if_can_not_parse_db_port() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "67000");

    let err = Config::init().err().unwrap();
    assert!(
        err.source().unwrap().is::<ParseIntError>(),
        format!("{:?}", &err.source())
    );
}

#[test]
fn test_custom_from_str() {
    use std::error::Error as StdError;

    setup();

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Envconfig for Point {
        fn parse(s: &str) -> Result<Self, Box<dyn StdError>> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        }
    }

    #[derive(Envconfig)]
    pub struct Config {
        #[envconfig(from = "DB_HOST")]
        point: Point,
    }

    env::set_var("DB_HOST", "(1,2)");

    let err = Config::init().unwrap();
    assert_eq!(err.point, Point { x: 1, y: 2 });
}

mod infer {
    use super::*;
    #[test]
    fn test_basic() {
        #[derive(Envconfig)]
        pub struct Config {
            user: String,
            pass: String,
        }

        let user = "root";
        let pass = "secret";
        env::set_var("USER", &user);
        env::set_var("PASS", &pass);

        let config = Config::init().unwrap();
        assert_eq!(config.user, user);
        assert_eq!(config.pass, pass);
    }

    #[test]
    fn test_nested() {
        #[derive(Envconfig)]
        pub struct DB {
            user: String,
            pass: String,
        }

        #[derive(Envconfig)]
        pub struct Config {
            db: DB,
            listen_port: u16,
        }

        let user = "root";
        let pass = "secret";
        let port = 1234;
        env::set_var("DB_USER", &user);
        env::set_var("DB_PASS", &pass);
        env::set_var("LISTEN_PORT", &port.to_string());

        let config = Config::init().unwrap();
        assert_eq!(config.db.user, user);
        assert_eq!(config.db.pass, pass);
        assert_eq!(config.listen_port, port);
    }

    #[test]
    fn test_optional() {
        #[derive(Envconfig)]
        pub struct Config {
            listen_port: Option<u16>,
            address: String,
        }

        let port = Some(1235u16);
        let address = "localhost";
        env::set_var("LISTEN_PORT", "1235");
        env::set_var("ADDRESS", &address.to_string());
        let config = Config::init().unwrap();
        assert_eq!(config.listen_port, port);
        assert_eq!(config.address, address);
    }
}
