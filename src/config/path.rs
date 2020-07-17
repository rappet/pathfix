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

impl From<SyntaxSuggarPath> for Path {
    fn from(path: SyntaxSuggarPath) -> Self {
        match path {
            SyntaxSuggarPath::Simple(s) => s.into(),
            SyntaxSuggarPath::Struct(s) => s
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum SyntaxSuggarPath {
    Simple(String),
    Struct(Path),
}

impl<S> From<S> for SyntaxSuggarPath
    where S: ToString {
    fn from(path: S) -> Self {
        SyntaxSuggarPath::Struct(path.to_string().into())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Paths(pub Vec<SyntaxSuggarPath>);

impl Paths {
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
        Paths(
            path.split(":")
                .map(SyntaxSuggarPath::from)
                .collect()
        )
    }

    /// Merges two `Paths` structures.
    /// Values `other` Config will be inserted before `self.
    pub fn merge(self, other: Paths) -> Paths {
        Paths(other.iter().chain(self.iter()).map(ToOwned::to_owned).collect())
    }
}

impl Deref for Paths {
    type Target = Vec<SyntaxSuggarPath>;

    fn deref(&self) -> &Vec<SyntaxSuggarPath> {
        &self.0
    }
}

impl DerefMut for Paths {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for Paths
    where T: ToString {
    fn from(v: Vec<T>) -> Self {
        Paths(
            v.iter()
                .map(ToString::to_string)
                .map(SyntaxSuggarPath::from)
                .collect()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::config::path::Paths;

    #[test]
    fn test_from_env() {
        use std::env;
        env::set_var("PATH", "/foo/bar:/fnorti/fnuff");
        let paths = Paths::from_env().unwrap();
        assert_eq!(paths.0, vec![
            "/foo/bar".into(),
            "/fnorti/fnuff".into()
        ]);
    }

    #[test]
    fn test_from_path() {
        let test_vec: Vec<(&'static str, Paths)> = vec![
            (
                "/foo/bar",
                vec!["/foo/bar"].into()
            ),
            (
                "/foo/bar:~/bin/bazz:$HOME/fnort/bar:${VAR}/some/path",
                vec!["/foo/bar", "~/bin/bazz", "$HOME/fnort/bar", "${VAR}/some/path"].into()
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
