use std::fmt::{self, Display, Formatter};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use crate::config::Config;
use std::env::VarError;
use std::ffi::OsStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConfigSource {
    PathVar,
    Included,
    System(ConfigFileFormat),
    Home(ConfigFileFormat),
    Config(ConfigFileDescription),
}

impl ConfigSource {
    pub fn open(self) -> io::Result<Config> {
        let mut config = match self {
            ConfigSource::PathVar => Config::new().with_env(),
            ConfigSource::Included => Config::included(),
            ConfigSource::System(_) | ConfigSource::Home(_) | ConfigSource::Config(_) => self
                .file_description()
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?
                .unwrap()
                .open()?,
        };
        config.paths.set_source(self);
        Ok(config)
    }

    /// Gets the file description, if the configuration source describes a file on the file system.
    fn file_description(&self) -> Result<Option<ConfigFileDescription>, VarError> {
        match self {
            ConfigSource::System(format) => Ok(Some(ConfigFileDescription::new(
                format!("/etc/pathfix.{}", format.extension()),
                *format,
            ))),
            ConfigSource::Home(format) => {
                let path: PathBuf = [
                    &std::env::var("HOME")?,
                    &format!(".pathfix.{}", format.extension()),
                ]
                .iter()
                .collect();
                Ok(Some(ConfigFileDescription::new(path, *format)))
            }
            ConfigSource::Config(description) => Ok(Some(description.clone())),
            _ => Ok(None),
        }
    }
}

impl Display for ConfigSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigSource::PathVar => write!(f, "PATH variable"),
            ConfigSource::Included => write!(f, "included in binary"),
            ConfigSource::System(_) => write!(f, "from system dir"),
            ConfigSource::Home(_) => write!(f, "from home dir"),
            ConfigSource::Config(description) => write!(f, "config: {}", description),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConfigFileDescription {
    location: PathBuf,
    format: ConfigFileFormat,
}

impl ConfigFileDescription {
    pub fn new<P: Into<PathBuf>>(location: P, format: ConfigFileFormat) -> ConfigFileDescription {
        ConfigFileDescription {
            location: location.into(),
            format,
        }
    }

    pub fn open(&self) -> io::Result<Config> {
        match self.format {
            ConfigFileFormat::Toml => Config::from_file(&self.location),
            ConfigFileFormat::Text => Config::from_txt(&self.location),
        }
    }
}

impl Display for ConfigFileDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.location.to_string_lossy())
    }
}

impl FromStr for ConfigFileDescription {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let location = PathBuf::from(s);

        Ok(match location.extension().and_then(OsStr::to_str) {
            Some("txt") => ConfigFileDescription::new(location, ConfigFileFormat::Text),
            Some("toml") => ConfigFileDescription::new(location, ConfigFileFormat::Toml),
            Some(extension) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("File extension {} not known", extension),
                ))
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Config file needs either 'toml' or 'txt' as file format",
                ))
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConfigFileFormat {
    Toml,
    Text,
}

impl ConfigFileFormat {
    fn extension(self) -> &'static str {
        match self {
            ConfigFileFormat::Toml => "toml",
            ConfigFileFormat::Text => "txt",
        }
    }
}
