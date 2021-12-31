use crate::context::Context;
use crate::Error;
use std::env;
use std::error::Error as StdError;
use std::fmt;

pub use bytesize::ByteSize;
pub use humantime::Duration;

#[derive(Debug)]
struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse method no implemented!")
    }
}

impl StdError for EmptyError {
    fn description(&self) -> &str {
        "Method parse is not implemented"
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

/// Indicates that structure can be initialize from environment variables.
pub trait Yasec {
    /// Creates empty context and calls `with_context`.
    fn init() -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::with_prefix("")
    }

    fn with_prefix(prefix: impl AsRef<str>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Self::with_context(Context::new(prefix))
    }

    /// Initialize structure from environment variable from the passed context.
    /// By default calls `parse` method. It works for a basic type like number or string.
    /// The method is redefined for a sctructure with `#[derive(Yasec)`. In that case
    /// the method pick every field type and calls the method for the type.
    fn with_context(context: Context) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let env_var_name = context.infer_var_name();
        match env::var(&env_var_name) {
            Ok(ref x) => Self::parse(x).map_err(|e| Error::new(e, env_var_name, x.to_owned())),
            Err(e) => match context.get_default_value() {
                Some(default) => Self::parse(&default)
                    .map_err(|e| Error::new(e, env_var_name, default.to_owned())),
                None => Err(Error::new(Box::new(e), env_var_name, None)),
            },
        }
    }

    /// Parses an environment variable value. It sould be implemented if an object is leaf of a
    /// configuration structure.
    fn parse(_val: &str) -> Result<Self, Box<dyn StdError>>
    where
        Self: Sized,
    {
        Err(Box::new(EmptyError))
    }

    fn usage() -> Result<String, Error>
    where
        Self: Sized,
    {
        Self::usage_prefix("")
    }

    fn usage_prefix(prefix: impl AsRef<str>) -> Result<String, Error>
    where
        Self: Sized,
    {
        Ok(Self::usage_with_context(Context::new(prefix))?
            .iter()
            .map(format_field_usage)
            .collect::<Vec<_>>()
            .join("\n"))
    }

    fn usage_with_context(context: Context) -> Result<Vec<Context>, Error>
    where
        Self: Sized,
    {
        Ok(vec![context])
    }
}

pub fn format_field_usage(context: &Context) -> String {
    format!(
        "{: <24}\t{:?}",
        context.infer_var_name(),
        context.get_default_value(),
    )
}

macro_rules! implement {
    ($x:ident) => {
        impl Yasec for $x {
            fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
                Ok(val.parse::<$x>()?)
            }
        }
    };
}

implement!(char);
implement!(u8);
implement!(u16);
implement!(u32);
implement!(u64);

implement!(i8);
implement!(i16);
implement!(i32);
implement!(i64);

implement!(f32);
implement!(f64);

implement!(bool); // "true" | "false"
implement!(Duration); // "60s"
implement!(ByteSize); // "1.50MB"

impl Yasec for String {
    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        Ok(val.to_owned())
    }
}

impl Yasec for Vec<String> {
    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        return Ok(val
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>());
    }
}

impl Yasec for Vec<i32> {
    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        let result = val
            .split(",")
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
            .map_err(|e| e.into());
        return result;
    }
}

impl<T: Yasec> Yasec for Option<T> {
    fn with_context(context: Context) -> Result<Self, Error> {
        let env_var_name = context.prefix();
        let env_var_result = env::var(&env_var_name);
        match env_var_result {
            Ok(ref x) => Self::parse(x).map_err(|e| Error::new(e, env_var_name, x.to_owned())),
            Err(_) => Ok(None),
        }
    }

    fn usage_with_context(context: Context) -> Result<Vec<Context>, Error> {
        Ok(vec![context])
    }

    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        Ok(Some(T::parse(val)?))
    }
}
