use std::convert::AsRef;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    var_name: String,
    var_value: Option<String>,
    origin: Option<Box<dyn StdError>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = self
            .origin
            .as_ref()
            .map(|x| format!("{}", x))
            .unwrap_or_default();
        write!(
            f,
            "Configuration from environment variables failed. Variable: {}.{} Reason: {}",
            self.var_name,
            self.var_value
                .as_ref()
                .map(|x| format!(" Value: {}.", x))
                .unwrap_or_default(),
            reason
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.origin.as_ref().map(|x| x.as_ref())
    }
}

impl Error {
    pub fn new<V, T>(source: Box<dyn StdError>, var_name: T, var_value: V) -> Self
    where
        T: AsRef<str>,
        V: Into<Option<String>>,
    {
        Self {
            origin: Some(source),
            var_name: var_name.as_ref().to_owned(),
            var_value: var_value.into(),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    failed_value: String,
}

impl ParseError {
    pub fn new(v: impl AsRef<str>) -> Self {
        Self {
            failed_value: v.as_ref().to_owned(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fail to parse {}", &self.failed_value)
    }
}

impl StdError for ParseError {}
