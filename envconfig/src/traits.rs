use context::Context;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use Error;

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
pub trait Envconfig {
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
    /// The method is redefined for a sctructure with `#[derive(Envconfig)`. In that case
    /// the method pick every field type and calls the method for the type.
    fn with_context(context: Context<Self>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut context = context;
        let env_var_name = context.infer_var_name();
        match env::var(&env_var_name) {
            Ok(ref x) => Self::parse(x).map_err(|e| Error::new(e, env_var_name, x.to_owned())),
            Err(e) => match context.take_default_var_value() {
                Some(default) => Ok(default),
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
}

macro_rules! implement {
    ($x:ident) => {
        impl Envconfig for $x {
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

impl Envconfig for String {
    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        Ok(val.to_owned())
    }
}

impl<T: Envconfig> Envconfig for Option<T> {
    fn with_context(context: Context<Self>) -> Result<Self, Error> {
        let env_var_name = context.prefix();
        let env_var_result = env::var(&env_var_name);
        match env_var_result {
            Ok(ref x) => Self::parse(x).map_err(|e| Error::new(e, env_var_name, x.to_owned())),
            Err(_) => Ok(None),
        }
    }

    fn parse(val: &str) -> Result<Self, Box<dyn StdError>> {
        Ok(Some(T::parse(val)?))
    }
}
