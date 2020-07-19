use serde::{Serialize, Deserialize};

mod path;

pub use path::{Path, SyntaxSuggarPath, Paths};
mod include_administrative;
pub use include_administrative::IncludeAdministrative;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use std::error::Error;
use serde::export::Formatter;
use core::fmt;

use std::io::Result as IoResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    // Do not read in higher directories
    #[serde(default)]
    pub base: bool,
    #[serde(default)]
    pub include_administrative: Option<IncludeAdministrative>,
    #[serde(default)]
    pub paths: Paths,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl Config {
    /// Creates a new Config
    ///
    /// This should yield the same as Default::default()
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Config::new(), Default::default());
    /// ```
    pub fn new() -> Config {
        Default::default()
    }

    pub fn included() -> Config {
        toml::from_str(include_str!("../config.toml")).unwrap()
    }

    /// Read the config from a specific file.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = Config::from_file("config.toml").unwrap();
    /// println!("{:?}", config);
    /// ```
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> IoResult<Config> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        Ok(toml::from_slice(&contents)?)
    }

    /// Sets the env parameter with the system environment.
    ///
    /// All existing variables in the config will be overwritten.
    /// Use `Config::merge` if you want to add them to existing
    /// environment variables in a `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    ///
    /// env::set_var("FOO", "BAR");
    /// let config = Config::new().with_env();
    /// assert_eq!(config.env["FOO"], "BAR");
    /// ```
    pub fn with_env(mut self) -> Config {
        self.env = std::env::vars().collect();
        self
    }

    /// Merges two `Config` structures.
    /// Changes in the `other` Config will overwrite
    /// vaules in `self`.
    pub fn merge(self, other: Config) -> Config {
        Config {
            base: self.base || other.base,
            include_administrative: other.include_administrative
                .or(self.include_administrative),
            paths: self.paths.merge(other.paths),
            env: self.env.into_iter().chain(other.env).collect()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConfigError {

}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError")
    }
}

impl Error for ConfigError {}

#[cfg(test)]
mod tests {
    use crate::config::{Config, IncludeAdministrative, Paths};

    #[test]
    fn test_new() {
        assert_eq!(Config::new(), Default::default());
    }

    #[test]
    fn test_with_env() {
        use std::env;
        env::set_var("FOO", "BAR");
        let config = Config::new().with_env();
        assert_eq!(config.env["FOO"], "BAR");
    }

    #[test]
    fn test_merge() {
        let config1 = Config {
            base: true,
            include_administrative: Some(IncludeAdministrative::Always),
            paths: vec!["/foo/bar", "/bar/bazz"].into(),
            env: vec![("FOO".to_string(), "BAR".to_string())].into_iter().collect()
        };
        let config2 = Config {
            base: true,
            include_administrative: Some(IncludeAdministrative::RootOnly),
            paths: Paths(vec!["/fnort".into()]),
            env: vec![("FOO".to_string(), "FNAFF".to_string())].into_iter().collect(),
        };
        let result = Config {
            base: true,
            include_administrative: Some(IncludeAdministrative::RootOnly),
            paths: vec!["/fnort", "/foo/bar", "/bar/bazz"].into(),
            env: vec![("FOO".to_string(), "FNAFF".to_string())].into_iter().collect()
        };
        assert_eq!(config1.merge(config2), result);
    }

    #[test]
    fn test_included() {
        Config::included();
    }
}
