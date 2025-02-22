use super::context::Context;
use super::YasecError;
use bytesize::ByteSize;
use humantime::Duration;
use regex::Regex;
use std::collections::HashMap;
use std::env;

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Indicates that structure can be initialize from environment variables.
pub trait Yasec {
    /// Creates empty context and calls `with_context`.
    fn init() -> Result<Self, YasecError>
    where
        Self: Sized,
    {
        Self::with_prefix("")
    }

    fn with_prefix(prefix: impl AsRef<str>) -> Result<Self, YasecError>
    where
        Self: Sized,
    {
        Self::with_context(Context::new(prefix))
    }

    /// Initialize structure from environment variable from the passed context.
    /// By default calls `parse` method. It works for a basic type like number or string.
    /// The method is redefined for a sctructure with `#[derive(Yasec)`. In that case
    /// the method pick every field type and calls the method for the type.
    fn with_context(context: Context) -> Result<Self, YasecError>
    where
        Self: Sized,
    {
        let env_var_name = context.infer_var_name();
        match env::var(&env_var_name) {
            Ok(ref value) => Self::parse(value).map_err(|e| YasecError::ParseEnvError {
                var_name: env_var_name,
                var_value: value.to_owned(),
                source: e,
            }),
            Err(e) => match context.get_default_value() {
                Some(default) => Self::parse(&default).map_err(|e| YasecError::ParseDefaultError {
                    var_name: env_var_name,
                    var_value: default.to_owned(),
                    source: e,
                }),
                None => match e {
                    env::VarError::NotPresent => Err(YasecError::EmptyVar(env_var_name)),
                    env::VarError::NotUnicode(_) => Err(YasecError::IllegalVar(env_var_name)),
                },
            },
        }
    }

    /// Parses an environment variable value. It sould be implemented if an object is leaf of a
    /// configuration structure.
    fn parse(_val: &str) -> Result<Self, StdError>
    where
        Self: Sized,
    {
        Err(Box::new(YasecError::IllegalVar("".to_owned())))
    }

    fn usage() -> Result<String, YasecError>
    where
        Self: Sized,
    {
        Self::usage_prefix("")
    }

    fn usage_prefix(prefix: impl AsRef<str>) -> Result<String, YasecError>
    where
        Self: Sized,
    {
        let header = format!("{: <24}\t{: <32}\t{}", "NAME", "TYPE", "DEFAULT");
        let delimiter = format!("{}", "-".repeat(90));
        let values = Self::usage_with_context(Context::new(prefix))?
            .iter()
            .map(format_field_usage)
            .collect::<Vec<_>>()
            .join("\n");
        Ok(format!("{}\n{}\n{}", header, delimiter, values))
    }

    fn usage_with_context(context: Context) -> Result<Vec<Context>, YasecError>
    where
        Self: Sized,
    {
        Ok(vec![context])
    }
}

pub fn format_field_usage(context: &Context) -> String {
    format!(
        "{: <24}\t{: <32}\t{}",
        context.infer_var_name(),
        format_usage_type(context),
        format_usage_default(context),
    )
}

// NOTE replace std::collections::HashMap to HashMap
lazy_static::lazy_static! {
    static ref TYPE_REMOVE_PREFIX_RE: Regex = Regex::new(r"(\w+::)+(\w+)").unwrap();
}

pub fn format_usage_type(context: &Context) -> String {
    TYPE_REMOVE_PREFIX_RE
        .replace_all(&context.get_var_type().replace(" ", ""), "$2")
        .to_string()
}

pub fn format_usage_default(context: &Context) -> String {
    match context.get_default_value() {
        Some(v) => v,
        None => "-".to_string(),
    }
}

macro_rules! implement {
    ($x:ident) => {
        impl Yasec for $x {
            fn parse(val: &str) -> Result<Self, StdError> {
                Ok(val.parse::<$x>()?)
            }
        }
    };
}

implement!(usize);
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
    fn parse(val: &str) -> Result<Self, StdError> {
        Ok(val.to_owned())
    }
}

impl Yasec for Vec<String> {
    fn parse(val: &str) -> Result<Self, StdError> {
        return Ok(val
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>());
    }
}

impl Yasec for Vec<i32> {
    fn parse(val: &str) -> Result<Self, StdError> {
        let result = val
            .split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
            .map_err(|e| e.into());
        result
    }
}

impl Yasec for HashMap<String, String> {
    fn parse(val: &str) -> Result<Self, StdError> {
        let v = val
            .split(',')
            .map(|s| match s.split_once('=') {
                Some((key, value)) => Ok((key.to_owned(), value.to_owned())),
                None => Err(YasecError::IllegalVar(s.to_owned())),
            })
            .collect::<Result<HashMap<_, _>, YasecError>>()?;
        Ok(v)
    }
}

impl<T: Yasec> Yasec for Option<T> {
    fn with_context(context: Context) -> Result<Self, YasecError> {
        let env_var_name = context.prefix();
        let found = std::env::vars()
            .filter(|(k, _v)| k.starts_with(&env_var_name))
            .collect::<Vec<(String, String)>>();

        if found.len() == 0 {
            return Ok(None);
        }

        Ok(Some(T::with_context(context)?))
    }

    fn usage_with_context(context: Context) -> Result<Vec<Context>, YasecError> {
        T::usage_with_context(context)
    }

    fn parse(val: &str) -> Result<Self, StdError> {
        Ok(Some(T::parse(val)?))
    }
}
