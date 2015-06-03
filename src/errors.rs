use std::error::Error;
use std::fmt;

pub type CrefResult<T> = Result<T, CrefError>;

#[derive(Debug)]
pub struct CrefError {
    pub error: Box<Error>
}

impl CrefError {
    pub fn from_error(error: Box<Error>) -> CrefError {
        CrefError { error: error }
    }
}

impl Error for CrefError {
    fn description(&self) -> &str { self.error.description() }
    fn cause(&self) -> Option<&Error> { self.error.cause() }
}

impl fmt::Display for CrefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.error, f)
    }
}
