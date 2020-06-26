#[macro_use]
extern crate yasec_derive;
extern crate yasec;

use yasec::Yasec;

#[derive(Yasec)]
pub struct Config {
    #[yasec(from = "PORT")]
    pub port: u16,

    #[yasec(from = "HOST")]
    pub host: String,
}

// Ensure custom Result can be defined in the current context.
// See: https://github.com/greyblake/yasec-rs/issues/21
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let res: Result<i32> = Ok(123);
    println!("{:?}", res);
}
