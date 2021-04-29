use std::fmt::Debug;
use std::env::VarError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error {
    #[from]
    repr: Repr,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
enum Repr {
    #[error("clap: {0}")]
    Clap(#[from] clap::Error),
    #[error(transparent)]
    Var(#[from] VarError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Repr::Clap(err).into()
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Repr::Var(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Repr::Io(err).into()
    }
}