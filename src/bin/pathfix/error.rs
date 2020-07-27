use std::fmt::{Display, Debug};
use serde::export::Formatter;
use core::fmt;
use std::env::VarError;
use std::io;

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Repr {
    Clap(clap::Error),
    Var(VarError),
    Io(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Clap(err) => write!(f, "clap: {}", err),
            Repr::Var(err) => Display::fmt(err, f),
            Repr::Io(err) => Display::fmt(err, f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match &self.repr {
            Repr::Clap(err) => err,
            Repr::Var(err) => err,
            Repr::Io(err) => err,
        })
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error{ repr: Repr::Clap(err) }
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Error {
        Error { repr: Repr::Var(err) }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error { repr: Repr::Io(err) }
    }
}
