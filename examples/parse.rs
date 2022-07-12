use bytesize::ByteSize;
use humantime::Duration;
use yasec::Yasec;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T> = std::result::Result<T, StdError>;

// TODO enums

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Yasec for Point {
    fn parse(s: &str) -> Result<Self> {
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

#[allow(dead_code)]
#[derive(Yasec, Debug)]
pub struct DB {
    #[yasec()]
    dsn: String,

    #[yasec()]
    secret: String,
}

#[allow(dead_code)]
#[derive(Yasec, Debug)]
pub struct Config {
    #[yasec(env = "PORT")]
    pub port: u16,

    #[yasec(env = "HOST")]
    pub host: String,

    #[yasec(env = "LABEL")]
    pub label: Option<String>,

    #[yasec(default = "123s")]
    pub default_ttl: Duration,

    #[yasec(env = "POINT")]
    pub point: Point,

    #[yasec(env = "MAYBE_DB")]
    pub maybe_db: Option<DB>,

    #[yasec(default = "15MB")]
    pub body_max_size: ByteSize,

    #[yasec(default = "true")]
    pub enabled: bool,

    pub db1: DB,
    pub db2: DB,

    #[yasec(default = "a,b,c")]
    pub string_list: Vec<String>,

    #[yasec(default = "1,2,3")]
    pub int_list: Vec<i32>,

    #[yasec(default = "a=b,c=d")]
    pub strstrmap: std::collections::HashMap<String, String>,
}

// Ensure custom Result can be defined in the current context.
// See: https://github.com/greyblake/yasec-rs/issues/21

fn main() {
    println!("{}", Config::usage().expect("usage"));
    // std::env::set_var("HOST", "localhost");
    // std::env::set_var("PORT", "1234");
    // std::env::set_var("POINT", "(1,2)");
    // std::env::set_var("LABEL", "val");
    // std::env::set_var("MAYBE_DB_DSN", "dsn:...");
    // std::env::set_var("MAYBE_DB_SECRET", "secret");

    let result = Config::init();
    match result {
        Ok(config) => {
            println!("config: {:#?}", config);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }
}
