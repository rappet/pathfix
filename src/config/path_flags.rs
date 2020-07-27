use std::fmt::Display;
use serde::export::Formatter;
use core::fmt;
use std::str::FromStr;
use std::io;
use crate::config::IncludeAdministrative;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use serde::de::Visitor;
use std::borrow::Borrow;


/// Flags for a Path
///
/// `PathFlags` describes the requirements that the system must fulfill
/// so that the `Path` will be included to the $PATH string.
///
/// # Examples
///
/// Parse the `PathFlags` and check the current system.
///
/// ```
/// use pathfix::config::{PathFlags, IncludeAdministrative};
///
/// let flags: PathFlags = "admin".parse().unwrap();
/// assert!(flags.check(PathFlags::this_system(&IncludeAdministrative::Always)));
/// ```
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PathFlags {
    admin: bool,
    os: PathOs,
}

impl PathFlags {
    /// Creates empty PathFlags without any requirements.
    ///
    /// # Examples
    /// ```
    /// use pathfix::config::PathFlags;
    ///
    /// println!("{}", PathFlags::new());
    /// ```
    pub fn new() -> PathFlags {
        Default::default()
    }

    /// Checks if the `PathFlags` are given for the flags of the given system.
    ///
    /// # Example
    /// Check if default flags are met.
    /// ```
    /// use pathfix::config::{PathFlags, IncludeAdministrative};
    /// assert!(
    ///     PathFlags::new().check(
    ///         PathFlags::this_system(&IncludeAdministrative::Always)
    ///     )
    /// );
    /// ```
    pub fn check(self, given: PathFlags) -> bool {
        let allow = !self.admin || given.admin;
        allow && given.os.is(self.os).unwrap_or(true)
    }

    /// Get given `PathFlags` for this system.
    ///
    /// # Example
    ///
    /// Test if current system is Linux.
    ///
    /// ```
    /// use pathfix::config::{PathFlags, IncludeAdministrative};
    ///
    /// let requirements: PathFlags = "linux".parse().unwrap();
    /// let this_system = PathFlags::this_system(&IncludeAdministrative::Always);
    ///
    /// assert_eq!(requirements.check(this_system), cfg!(target_os = "linux"));
    /// ```
    pub fn this_system<I>(include_administrative: &I) -> PathFlags
        where I: Borrow<IncludeAdministrative> {
        PathFlags {
            admin: include_administrative.borrow().check_current_user().unwrap_or(true),
            os: PathOs::this_system(),
        }
    }
}

impl Display for PathFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let os = self.os.to_string();
        let mut parts = Vec::new();
        if self.admin {
            parts.push("admin");
        }
        parts.push(&os);
        write!(f, "{}", &parts.join(","))
    }
}

impl Serialize for PathFlags {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

struct PathFlagsVisitor;

impl<'de> Visitor<'de> for PathFlagsVisitor {
    type Value = PathFlags;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("a list of path flags separated by a , as a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: de::Error, {
        PathFlags::from_str(v).map_err(|e| E::custom(
            format!("error parsing path flags: {}", e.to_string())
        ))
    }
}

impl<'de> Deserialize<'de> for PathFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_str(PathFlagsVisitor)
    }
}

impl FromStr for PathFlags {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut os_set = false;
        let mut flags = PathFlags::new();
        for flag in s.split(',')
            .map(str::trim) {
            if flag.eq_ignore_ascii_case("admin") {
                if flags.admin {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "admin flag is already set"
                    ));
                }
                flags.admin = true;
            } else if let Ok(os) = PathOs::from_str(flag) {
                if os_set {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "os is already set"
                    ));
                }
                os_set = true;
                flags.os = os;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unkown path flag {}", flag)
                ));
            }
        }
        Ok(flags)
    }
}

/// Operating system requirements
///
/// `PathOs` describes the requirements for an operating system or
/// it itself describes the local operating system.
///
/// # Examples
///
/// ```
/// use pathfix::config::PathOs;
///
/// let linux:   PathOs = "linux".parse().unwrap();
/// let windows: PathOs = "windows".parse().unwrap();
/// let osx:     PathOs = "osx".parse().unwrap();
/// let unix:    PathOs = "unix".parse().unwrap();
/// let any = PathOs::default();
///
/// assert_eq!(linux.is(linux).unwrap(), true);
/// assert_eq!(linux.is(unix).unwrap(), true);
/// assert_eq!(osx.is(unix).unwrap(), true);
/// assert_eq!(windows.is(unix).unwrap(), false);
/// assert_eq!(windows.is(any).unwrap(), true);
/// ```
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum PathOs {
    Any,
    Unix,
    Windows,
    Linux,
    OSX,
    Unknown,
}

impl PathOs {
    /// Returns the current operating system
    pub fn this_system() -> PathOs {
        if cfg!(target_os = "linux") {
            PathOs::Linux
        } else if cfg!(windows) {
            PathOs::Windows
        } else if cfg!(target_os = "macos") {
            PathOs::OSX
        } else if cfg!(unix) {
            PathOs::Unix
        } else {
            warn!("Unknown operating system. // TODO bug report");
            PathOs::Unknown
        }
    }

    /// Checks if `self` is the specified operating system or belongs
    /// to the specified operating system group
    pub fn is(self, other: PathOs) -> PathOsResult<bool> {
        match (self, other) {
            (PathOs::Windows, PathOs::Windows) => Ok(true),
            (_, PathOs::Unix) => self.is_unix(),
            (_, PathOs::Unknown) => Err(PathOsError::CheckAgainstUnknownOs),
            (a, b) if a == b => Ok(true),
            (_, PathOs::Any) => Ok(true),
            (_, _) => Ok(false),
        }
    }

    /// Check if the OS is a UNIX
    ///
    /// # Example
    ///
    /// ```
    /// use pathfix::config::PathOs;
    ///
    /// assert_eq!(PathOs::Unix.is_unix().unwrap(), true);
    /// assert_eq!(PathOs::Windows.is_unix().unwrap(), false);
    /// assert_eq!(PathOs::OSX.is_unix().unwrap(), true);
    /// ```
    pub fn is_unix(self) -> PathOsResult<bool> {
        match self {
            PathOs::Unix | PathOs::Linux | PathOs::OSX => Ok(true),
            PathOs::Windows => Ok(false),
            PathOs::Any => Err(PathOsError::CheckAnyOs),
            PathOs::Unknown => Err(PathOsError::CheckUnknownOs),
        }
    }
}

impl Default for PathOs {
    fn default() -> PathOs {
        PathOs::Any
    }
}

impl fmt::Display for PathOs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PathOs::Any => "any",
            PathOs::Unix => "unix",
            PathOs::Windows => "windows",
            PathOs::Linux => "linux",
            PathOs::OSX => "osx",
            PathOs::Unknown => "unknown",
        })
    }
}

impl FromStr for PathOs {
    type Err = ParsePathOsError;

    fn from_str(s: &str) -> ParsePathOsResult {
        Ok(match s.to_ascii_lowercase().as_str() {
            "" | "any" => PathOs::Any,
            "unix" => PathOs::Unix,
            "windows" => PathOs::Windows,
            "win" | "win32" => {
                warn!("You should refer to Windows with 'windows' in your configuration, not with '{}'", s);
                PathOs::Windows
            }
            "linux" => PathOs::Linux,
            "osx" => PathOs::OSX,
            "mac" | "macos" | "macosx" => {
                warn!("You should refer to MacOSX with 'osx' in your configuration, not with '{}'", s);
                PathOs::OSX
            }
            _ => return Err(ParsePathOsError { name: s.to_string() } )
        })
    }
}

/// Error type used by the `PathOs` type.
///
/// # Examples
///
/// ```
/// use pathfix::config::{PathOs, PathOsError};
///
/// assert_eq!(PathOs::Any.is(PathOs::Unix), Err(PathOsError::CheckAnyOs));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PathOsError {
    CheckAnyOs,
    CheckUnknownOs,
    CheckAgainstUnknownOs,
}

pub type PathOsResult<T> = std::result::Result<T, PathOsError>;

impl Display for PathOsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PathOsError::CheckAnyOs => f.write_str("Cannot check if 'Any' OS belongs to another OS group"),
            PathOsError::CheckUnknownOs => f.write_str("Cannot check if 'Unknown' OS belongs to another OS group"),
            PathOsError::CheckAgainstUnknownOs => f.write_str("Cannot check if OS against a unknown OS"),
        }
    }
}

impl std::error::Error for PathOsError {}

/// Error type which describes which operating system is unknown
#[derive(Debug)]
pub struct ParsePathOsError {
    name: String,
}

pub type ParsePathOsResult = std::result::Result<PathOs, ParsePathOsError>;

impl Display for ParsePathOsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' is not a known operating system", self.name)
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    use std::str::FromStr;

    use crate::config::PathFlags;

    #[derive(Serialize, Deserialize)]
    struct TestType {
        pub flags: PathFlags
    }

    #[test]
    fn test_serialize() {
        let input = TestType { flags: PathFlags::from_str("osx, admin").unwrap() };
        let wanted = "flags = 'admin,osx'\n";
        assert_eq!(&toml::to_string_pretty(&input).unwrap(), wanted);
    }

    #[test]
    fn test_deserialize() {
        let input: TestType = toml::from_str("flags = \"osx, admin\"\n").unwrap();
        let wanted: PathFlags = "admin,osx".parse().unwrap();
        assert_eq!(input.flags, wanted);
    }
}
