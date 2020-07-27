//! Configuration file and Path business logic

use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

mod include_administrative;

pub use include_administrative::IncludeAdministrative;

mod path;

pub use path::{Path, Paths, ConfigSource};

mod path_flags;

pub use path_flags::{PathFlags, PathOs, PathOsError, PathOsResult, ParsePathOsError, ParsePathOsResult};
use std::path::PathBuf;
use std::str::FromStr;

/// Main configuration file
///
/// The main purpose of this config is to provide `Paths` which should be
/// added to the _$PATH_ variable.
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
    /// use pathfix::config::Config;
    ///
    /// assert_eq!(Config::new(), Default::default());
    /// ```
    pub fn new() -> Config {
        Default::default()
    }

    pub fn included() -> Config {
        let mut config: Config = toml::from_str(include_str!("../config.toml")).unwrap();
        config.paths.set_source(ConfigSource::Included);
        config
    }

    /// Read the config from a specific file.
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfix::config::Config;
    ///
    /// let config = Config::from_file("src/config.toml").unwrap();
    /// println!("{:?}", config);
    /// ```
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> io::Result<Config> {
        Config::from_file_inner(&path).map_err(
            |err| io::Error::new(err.kind(), format!("{}: {}",
                                                     path.as_ref().to_string_lossy(),
                                                     err.to_string()
            ))
        )
    }

    fn from_file_inner<P: AsRef<std::path::Path>>(path: &P) -> io::Result<Config> {
        let path_ref = path.as_ref();
        let mut file = File::open(&path_ref)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let mut config: Config = toml::from_slice(&contents)?;
        config.paths.set_source(ConfigSource::Config(PathBuf::from(path_ref)));
        Ok(config)
    }

    /// Read the config from a simple txt file
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfix::config::Config;
    ///
    /// Config::from_txt("src/config.txt").unwrap();
    /// ```
    pub fn from_txt<P: AsRef<std::path::Path>>(path: P) -> io::Result<Config> {
        let path_ref = path.as_ref();
        let mut file = File::open(&path_ref)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let contents_str = String::from_utf8(contents).map_err(
            |err| io::Error::new(io::ErrorKind::InvalidData, err.to_string())
        )?;

        let paths: io::Result<Vec<_>> = contents_str.lines()
            .map(|line| line.splitn(2, '#').next().unwrap())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|line| Path::from_str(line))
            .collect();

        Ok(Config {
            paths: Paths::new(paths?),
            ..Default::default()
        })
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
    /// use pathfix::config::Config;
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
            env: self.env.into_iter().chain(other.env).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, IncludeAdministrative, Paths};
    use crate::config::path::ConfigSource;

    #[test]
    fn test_new() {
        assert_eq!(Config::new(), Default::default());
    }

    #[test]
    fn test_from_file() {
        let mut config = Config::from_file("src/config.toml").unwrap();
        config.paths.set_source(ConfigSource::Included);
        let wanted = Config::included();
        assert_eq!(config, wanted);
    }

    #[test]
    fn test_from_txt() {
        let from_toml = Config::from_file("src/config.toml").unwrap();
        let from_txt = Config::from_txt("src/config.txt").unwrap();

        assert_eq!(from_toml.paths.normalize(), from_txt.paths.normalize());
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
            env: vec![("FOO".to_string(), "BAR".to_string())].into_iter().collect(),
        };
        let config2 = Config {
            base: true,
            include_administrative: Some(IncludeAdministrative::RootOnly),
            paths: Paths::from(vec!["/fnort"]),
            env: vec![("FOO".to_string(), "FNAFF".to_string())].into_iter().collect(),
        };
        let result = Config {
            base: true,
            include_administrative: Some(IncludeAdministrative::RootOnly),
            paths: vec!["/fnort", "/foo/bar", "/bar/bazz"].into(),
            env: vec![("FOO".to_string(), "FNAFF".to_string())].into_iter().collect(),
        };
        assert_eq!(config1.merge(config2), result);
    }

    #[test]
    fn test_included() {
        Config::included();
    }
}
