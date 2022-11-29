// Most of this is lifted from io::error, per suggestion at
//   https://learning-rust.github.io/docs/e7.custom_error_types.html

use std::error;
use std::error::Error;
use std::fmt;
use std::io;
use tracing::debug;
use csv::Error as CSVError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrorKind {
    InvalidArgs,
    InvalidCSV,
    InvalidYaml,
    IoError,
    ExecutionError,
}

impl ErrorKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::InvalidArgs => "arguments provided are invalid",
            ErrorKind::InvalidCSV => "insufficient or malformed csv provided",
            ErrorKind::InvalidYaml => "insufficient or malformed yaml provided",
            ErrorKind::IoError => "could not perform io",
            ErrorKind::ExecutionError => "command module returned non-zero status code",
        }
    }
}

pub struct TranscriberError {
    repr: Repr,
}

impl fmt::Debug for TranscriberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl fmt::Display for TranscriberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Simple(kind) => write!(f, "{}", kind.as_str()),
            Repr::Custom(err) => {
                write!(f, "{}: {}", err.as_ref().kind.as_str(), err.as_ref().error)
            }
        }
    }
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<dyn Error + Send + Sync>,
}

impl From<ErrorKind> for TranscriberError {
    #[inline] // This is in io::error, and presumably is efficient to use here.
    fn from(kind: ErrorKind) -> TranscriberError {
        TranscriberError {
            repr: Repr::Simple(kind),
        }
    }
}

impl From<io::Error> for TranscriberError {
    #[inline]
    fn from(err: io::Error) -> TranscriberError {
        TranscriberError::new(ErrorKind::IoError, err)
    }
}

impl From<csv::Error> for TranscriberError {
    #[inline]
    fn from(err: csv::Error) -> TranscriberError {
        TranscriberError::new(ErrorKind::InvalidCSV, err)
    }
}

impl TranscriberError {
    pub fn new<E>(kind: ErrorKind, error: E) -> TranscriberError
        where
            E: Into<Box<dyn Error + Send + Sync>>,
    {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ErrorKind, error: Box<dyn error::Error + Send + Sync>) -> TranscriberError {
        TranscriberError {
            repr: Repr::Custom(Box::new(Custom { kind, error })),
        }
    }
}

pub fn csv_err(e: &dyn Error, msg: &str) -> TranscriberError {
    debug!("{}", e);
    TranscriberError::new(ErrorKind::InvalidCSV, msg)
}


pub fn io_err(e: &dyn Error, msg: &str) -> TranscriberError {
    debug!("{}", e);
    TranscriberError::new(ErrorKind::IoError, msg)
}

pub fn yaml_err(e: &dyn Error, msg: &str) -> TranscriberError {
    debug!("{}", e);
    TranscriberError::new(ErrorKind::InvalidYaml, msg)
}
