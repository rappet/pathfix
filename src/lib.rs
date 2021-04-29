//! This is the library path of *PathFIX*.
//!
//! Because documentation tests don't work in rust if the crate is a binary crate,
//! this crate is implemented as a library with an added binary.

extern crate serde;
extern crate toml;
extern crate users;
#[macro_use]
extern crate log;
extern crate thiserror;

pub mod config;
