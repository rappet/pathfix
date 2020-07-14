use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    // Do not read in higher directories
    pub base: bool,
    pub include_administrative: Option<IncludeAdministrative>,
    #[serde(default)]
    pub paths: Vec<SyntaxSuggarPath>,
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
        serde_yaml::from_str(include_str!("config.yml")).unwrap()
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

    /// Reads PATH environment variable file and adds content to config.
    ///
    /// The PATH environment variable will be split on ':' and
    /// replace all paths in the configuration.
    ///
    /// If the variables should be added and not replaces,
    /// combine with `Config::merge`
    pub fn with_path_from_env(mut self) -> Result<Config, &'static str> {
        self.paths = std::env::var("PATH").map_err(|_| "$PATH does not contain valid utf8")?
            .split(":")
            .map(SyntaxSuggarPath::from)
            .collect();
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IncludeAdministrative {
    Always,
    RootOnly,
    Groups(Vec<String>),
    Never
}

impl Default for IncludeAdministrative {
    fn default() -> Self {
        IncludeAdministrative::Groups(vec![
            "wheel".to_string(),
            "sudo".to_string(),
        ])
    }
}

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
            SyntaxSuggarPath::Simple(s) =>  s.into(),
            SyntaxSuggarPath::Struct(s) => s
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum SyntaxSuggarPath {
    Simple(String),
    Struct(Path)
}

impl<S> From<S> for SyntaxSuggarPath
where S: ToString {
    fn from(path: S) -> Self {
        SyntaxSuggarPath::Struct(path.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

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
    fn test_with_path_from_env() {
        use std::env;
        env::set_var("PATH", "/foo/bar:/fnorti/fnuff");
        let config = Config::new().with_path_from_env().unwrap();
        assert_eq!(config.paths, vec![
            "/foo/bar".into(),
            "/fnorti/fnuff".into()
        ]);
    }

    #[test]
    fn test_included() {
        Config::included();
    }
}
