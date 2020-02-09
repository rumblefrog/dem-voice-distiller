use std::io::Error as IOError;

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IOError(IOError),
    InvalidHeader,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Error::IOError(e) => write!(f, "IO error occurred: {}", e),
            Error::InvalidHeader => write!(f, "Demo file contains invalid header"),
        }
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::IOError(err)
    }
}