use thiserror::Error;

#[derive(Error, Debug)]
pub enum YasecError {
    #[error("Configuration from environment variables failed. Variable: `{var_name}` with value `{var_value}`")]
    ParseEnvError {
        var_name: String,
        var_value: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("Configuration from environment variables failed. Variable: `{var_name}` with default value `{var_value}`")]
    ParseDefaultError {
        var_name: String,
        var_value: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error(
        "Configuration from environment variables failed. Environment variable: {0} not present"
    )]
    EmptyVar(String),
    #[error("Configuration from environment variables failed. Environment variable `{0}` contains illegal value")]
    IllegalVar(String),
}
