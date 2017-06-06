#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(custom)]
    #[error_chain(description = r#"|| "Could not find a player""#)]
    PlayerNotFound,

    #[error_chain(custom)]
    #[error_chain(description = r#"|e| e"#)]
    IdError(String),

    #[error_chain(foreign)]
    ParseError(::std::num::ParseFloatError),

    #[error_chain(foreign)]
    DbTimeout(::r2d2::GetTimeout),

    #[error_chain(foreign)]
    DbError(::postgres::error::Error),
}
