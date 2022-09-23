use thiserror::Error;

use derivative::*;

#[derive(Derivative, Error)]
#[derivative(Debug, PartialEq)]
pub enum YasecError {
    #[error("Configuration from environment variables failed. Variable: `{var_name}` with value `{var_value}`, {source}")]
    ParseEnvError {
        var_name: String,
        var_value: String,
        #[derivative(PartialEq = "ignore")]
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("Configuration from environment variables failed. Variable: `{var_name}` with default value `{var_value}`, {source}")]
    ParseDefaultError {
        var_name: String,
        var_value: String,
        #[derivative(PartialEq = "ignore")]
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error(
        "Configuration from environment variables failed. Environment variable: {0} not present"
    )]
    EmptyVar(String),
    #[error("Illegal value `{0}`")]
    IllegalVar(String),
}
