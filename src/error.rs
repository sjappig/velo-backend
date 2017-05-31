use std::fmt;

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)]
    ParseError(::std::num::ParseFloatError),

    #[error_chain(foreign)]
    DbTimeout(::r2d2::GetTimeout),

    #[error_chain(foreign)]
    DbError(::postgres::error::Error),
}

/*
impl fmt::Display for VeloError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let err = match *self {
            VeloError::DbError(ref err) => err,
        };
        write!(fmt, "Velo error: {}", err)
    }
}
*/
