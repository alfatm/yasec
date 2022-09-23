//! Yasec is a Rust library that helps to initialize configuration structure
//! from environment variables.
//! It makes use of custom derive macros to reduce boilerplate.
//!
//! Example
//!
//! ```
//! use yasec::*;
//! use std::env;
//!
//! #[derive(Yasec)]
//! struct Config {
//!     #[yasec(env = "DB_HOST")]
//!     pub db_host: String,
//!
//!     #[yasec(env = "DB_PORT")]
//!     pub db_port: Option<u16>,
//!
//!     #[yasec(env = "HTTP_PORT", default = "8080")]
//!     pub http_port: u16,
//! }
//!
//! fn main() {
//!     // We assume that those environment variables are set somewhere outside
//!     env::set_var("DB_HOST", "localhost");
//!     env::set_var("DB_PORT", "5432");
//!
//!     // Initialize config from environment variables
//!     let config = Config::init().unwrap();
//!
//!     assert_eq!(config.db_host, "localhost");
//!     assert_eq!(config.db_port, Some(5432));
//!     assert_eq!(config.http_port, 8080);
//! }
//! ```
//!
//! The library uses `std::str::FromStr` trait to convert environment variables into custom
//! data type. So, if your data type does not implement `std::str::FromStr` the program
//! will not compile.

mod context;
mod error;
mod traits;

pub use context::*;
pub use error::*;
pub use traits::*;
pub use yasec_derive::*;
