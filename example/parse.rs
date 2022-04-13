use yasec::Yasec;

use bytesize::ByteSize;
use humantime::Duration;

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
}

// Ensure custom Result can be defined in the current context.
// See: https://github.com/greyblake/yasec-rs/issues/21

fn main() {
    println!("{}", Config::usage().expect("usage"));

    let config = Config::init();
    println!("{:#?}", config);

    let res: Result<i32> = Ok(123);
    println!("{:?}", res);
}
