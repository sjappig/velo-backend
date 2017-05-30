use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VeloError {
    DbError(String),
}

impl fmt::Display for VeloError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let err = match *self {
            VeloError::DbError(ref err) => err,
        };
        write!(fmt, "Velo error: {}", err)
    }
}

impl Error for VeloError {
    fn description(&self) -> &str {
        match *self {
            VeloError::DbError(ref err) => err,
        }
    }
}
