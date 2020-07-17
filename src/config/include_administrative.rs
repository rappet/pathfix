use serde::{Serialize, Deserialize};

use std::string::ToString;

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
