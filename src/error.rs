use thiserror::Error;

#[derive(Error, Debug)]
pub enum YasecError {
    #[error("unable parse variable")]
    ParseError {
        var_name: String,
        var_value: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("illegal variable value")]
    IllegalVar(String),
    #[error("empty variable")]
    EmptyVar,
    #[error("unknown error")]
    Unknown,
}
