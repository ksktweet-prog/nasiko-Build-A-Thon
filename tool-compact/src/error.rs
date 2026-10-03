use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("unknown tool")]
    UnknownTool,

    #[error("unterminated call")]
    UnterminatedCall,

    #[error("invalid json")]
    InvalidJson,

    #[error("invalid arguments")]
    InvalidArguments,

    #[error("invalid syntax")]
    InvalidSyntax,
}