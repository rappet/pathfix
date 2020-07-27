use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use std::env::VarError;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Path {
    pub path: String,
    pub admin: bool,
}

impl Path {
    pub fn resolve(&self, env: &HashMap<String, String>) -> Option<String> {
        let path: Option<Vec<String>> = self.path
            .split('/')
            .map(|folder| {
                if folder == "~" {
                    env.get("HOME").map(String::to_string)
                } else if folder.chars().next() == Some('$') {
                    env.get(&folder[1..]).map(String::to_string)
                } else {
                    Some(folder.to_string())
                }
            })
            .collect();
        path.map(|path| path.join("/"))
    }

    /// Returns the contained path string
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn flags(&self) -> PathFlags {
        self.flags
    }

    /// Returns the source of where the path originates from
    pub fn source(&self) -> Option<&Rc<ConfigSource>> {
        self.source.as_ref()
    }
}

impl<S> From<S> for Path
    where S: Into<String> {
    fn from(s: S) -> Self {
        Path {
            path: s.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConfigSource {
    PathVar,
    Included,
    Config(std::path::PathBuf),
}

impl Display for ConfigSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConfigSource::PathVar => write!(f, "PATH variable"),
            ConfigSource::Included => write!(f, "included in binary"),
            ConfigSource::Config(config) => write!(f, "config: {}", config.to_string_lossy()),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Paths(pub Vec<Path>);

impl Paths {
    pub fn new(v: Vec<Path>) -> Paths {
        Paths(v)
    }

    /// Reads PATH environment variable file and adds content to config.
    ///
    /// The PATH environment variable will be split on ':'
    pub fn from_env() -> Result<Paths, VarError> {
        Ok(Paths::from_path(
            &std::env::var("PATH")?
        ))
    }

    /// Parses a colon seperated path and sets that as included path.
    ///
    /// The parameter will be split on ':'
    pub fn from_path(path: &str) -> Paths {
        let config_source = Rc::new(ConfigSource::PathVar);
        Paths(
            path.split(':')
                .map(Path::from)
                .map(|mut path| {
                    path.source = Some(config_source.clone());
                    path
                })
                .collect()
        )
    }

    /// Merges two `Paths` structures.
    /// Values `other` Config will be inserted before `self.
    pub fn merge(self, other: Paths) -> Paths {
        Paths(other.0.iter().chain(self.0.iter()).map(ToOwned::to_owned).collect())
    }

    pub fn resolve(&self, system_flags: PathFlags, env: &HashMap<String, String>) -> Vec<String> {
        self.0.iter()
            .cloned()
            .filter(|p| p.flags.check(system_flags))
            .filter_map(|p| p.resolve(env))
            .collect()
    }

    /// Sets the source of all paths in the internal vector.
    pub fn set_source(&mut self, source: ConfigSource) {
        let rc = Rc::new(source);
        for path in self.0.iter_mut() {
            path.source = Some(rc.clone());
        }
    }
}

impl Serialize for Paths {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer,
    {
        let mut serialize_map = serializer.serialize_map(Some(self.0.len()))?;
        for path in &self.0 {
            serialize_map.serialize_entry(&path.path, &path.flags)?;
        }
        serialize_map.end()
    }
}

struct PathsVisitor;

impl<'de> Visitor<'de> for PathsVisitor {
    type Value = Paths;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("an map that maps paths to their flags as strings")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Paths, A::Error> where
        A: MapAccess<'de>, {
        let mut paths = Vec::new();
        while let Some((path, flags)) = map.next_entry()? {
            paths.push(Path {
                path, flags, ..Default::default()
            })
        }
        Ok(Paths(paths))
    }
}

impl<'de> Deserialize<'de> for Paths {
    fn deserialize<D>(deserializer: D) -> Result<Paths, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_map(PathsVisitor)
    }
}

impl<T> From<Vec<T>> for Paths
    where T: ToString {
    fn from(v: Vec<T>) -> Self {
        Paths(
            v.iter()
                .map(ToString::to_string)
                .map(Path::from)
                .collect()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::string::ToString;
    use std::rc::Rc;

    use crate::config::{Path, Paths, PathFlags, ConfigSource};

    #[test]
    fn test_resolve() {
        let env: HashMap<String, String> = [("HOME", "/home/user"), ("FOO", "/foobar")]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let testvec = [
            (
                Path::from("/fnort/bar"),
                Some("/fnort/bar".to_string())
            ),
            (
                Path::from("$HOME/foo"),
                Some("/home/user/foo".to_string())
            ),
            (
                Path::from("~/foo"),
                Some("/home/user/foo".to_string())
            ),
            (
                Path::from("$UNKOWN/foo"),
                None
            ),
        ];

        for (path, wanted) in &testvec {
            assert_eq!(path.resolve(&env), *wanted);
        }
    }

    #[test]
    fn test_from_env() {
        use std::env;
        env::set_var("PATH", "/foo/bar:/fnorti/fnuff");
        let paths = Paths::from_env().unwrap();
        assert_eq!(paths, Paths::new(vec![
            Path::with_source("/foo/bar", PathFlags::default(), ConfigSource::PathVar),
            Path::with_source("/fnorti/fnuff", PathFlags::default(), ConfigSource::PathVar),
        ]));
    }

    #[test]
    fn test_from_path() {
        let source = Rc::new(ConfigSource::PathVar);
        let test_vec: Vec<(&'static str, Paths)> = vec![
            (
                "/foo/bar",
                Paths::new(vec![
                    Path::with_source("/foo/bar", PathFlags::default(), ConfigSource::PathVar)
                ])
            ),
            (
                "/foo/bar:~/bin/bazz:$HOME/fnort/bar:${VAR}/some/path",
                Paths::new(vec![
                    Path::with_source("/foo/bar", PathFlags::default(), source.clone()),
                    Path::with_source("~/bin/bazz", PathFlags::default(), source.clone()),
                    Path::with_source("$HOME/fnort/bar", PathFlags::default(), source.clone()),
                    Path::with_source("${VAR}/some/path", PathFlags::default(), source.clone())
                ])
            ),
        ];

        for (path, paths) in test_vec {
            assert_eq!(Paths::from_path(path), paths);
        }
    }

    #[test]
    fn test_merge() {
        let test_vec: Vec<(Paths, Paths, Paths)> = vec![
            (Paths::default(), Paths::default(), Paths::default()),
            (
                vec!["/foo/bar", "/bar/bazz"].into(),
                vec!["/fnort"].into(),
                vec!["/fnort", "/foo/bar", "/bar/bazz"].into(),
                )
        ];

        for (a, b, out) in test_vec {
            assert_eq!(a.merge(b), out)
        }
    }
}
