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

#[cfg(test)]
mod tests {
    use crate::config::IncludeAdministrative;

    #[test]
    fn test_default() {
        let default = IncludeAdministrative::default().clone();
        assert_eq!(
            default,
            IncludeAdministrative::Groups(vec![
                "wheel".to_string(),
                "sudo".to_string(),
            ])
        )
    }
}
