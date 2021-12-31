#[macro_use]
extern crate yasec_derive;
extern crate yasec;

use std::env;
use std::error::Error as _;
use std::num::ParseIntError;

use yasec::Yasec;

#[derive(Yasec)]
pub struct Config {
    #[yasec(env = "DB_HOST")]
    pub db_host: String,

    #[yasec(env = "DB_PORT")]
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
        "{:?}",
        &err.source()
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

    impl Yasec for Point {
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

    #[derive(Yasec)]
    pub struct Config {
        #[yasec(env = "DB_HOST")]
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
        #[derive(Yasec)]
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
        #[derive(Yasec)]
        pub struct DB {
            user: String,
            pass: String,
        }

        #[derive(Yasec)]
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
        #[derive(Yasec)]
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

    #[test]
    fn test_bool() {
        #[derive(Yasec)]
        pub struct Config {
            enabled: bool,
        }

        env::set_var("ENABLED", "true");
        let config = Config::init().unwrap();
        assert_eq!(config.enabled, true);
    }

    #[test]
    fn test_string_vector() {
        #[derive(Yasec)]
        pub struct Config {
            #[yasec(default = "a,b,c")]
            pub string_list: Vec<String>,
        }

        let config = Config::init().unwrap();
        assert_eq!(config.string_list, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_int_vector() {
        #[derive(Yasec)]
        pub struct Config {
            #[yasec(default = "1,2,3")]
            pub int_list: Vec<i32>,
        }

        let config = Config::init().unwrap();
        assert_eq!(config.int_list, vec![1, 2, 3]);
    }

    #[test]
    fn test_size_units() {
        use bytesize::ByteSize;

        #[derive(Yasec)]
        pub struct Config {
            pub body_max_size: ByteSize,
        }

        env::set_var("BODY_MAX_SIZE", "15MB");
        let config = Config::init().unwrap();
        assert_eq!(config.body_max_size, ByteSize::mb(15));
    }

    #[test]
    fn test_duration() {
        use humantime::Duration;
        #[derive(Yasec)]
        pub struct Config {
            some_ttl: Duration,

            #[yasec(default = "567s")]
            default_ttl: Duration,
        }

        env::set_var("SOME_TTL", "123s");
        env::remove_var("DEFAULT_TTL");

        let config = Config::init().unwrap();
        assert_eq!(
            *config.some_ttl.as_ref(),
            std::time::Duration::from_secs(123)
        );
        assert_eq!(
            *config.default_ttl.as_ref(),
            std::time::Duration::from_secs(567)
        );
    }
}
