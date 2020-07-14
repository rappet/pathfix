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
    pub fn included() -> Config {
        serde_yaml::from_str(include_str!("config.yml")).unwrap()
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
    fn test_included() {
        Config::included();
    }
}
