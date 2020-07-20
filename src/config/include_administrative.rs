use std::io;
use std::string::ToString;

use serde::{Deserialize, Serialize};
use users::Group;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IncludeAdministrative {
    Always,
    RootOnly,
    Groups(Vec<String>),
    Never,
}

impl IncludeAdministrative {
    /// `check_current_user` will check if the running users qualifies as an administrator.
    ///
    /// The result will be used to includes path's which are marked as admin only.
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfix::config::IncludeAdministrative;
    ///
    /// assert_eq!(IncludeAdministrative::Always.check_current_user().unwrap(), true);
    /// assert_eq!(IncludeAdministrative::Never.check_current_user().unwrap(), false);
    /// assert_eq!(IncludeAdministrative::RootOnly.check_current_user().unwrap(), users::get_current_uid() == 0);
    /// ```
    pub fn check_current_user(&self) -> io::Result<bool> {
        Ok(match self {
            IncludeAdministrative::Always => true,
            IncludeAdministrative::RootOnly => users::get_current_uid() == 0,
            // TODO add test for Group check
            IncludeAdministrative::Groups(groups) => {
                for group in users::group_access_list()?
                    .iter()
                    .map(Group::name)
                    .filter_map(|g| g.to_str())
                    {
                        if groups.contains(&group.to_string()) {
                            return Ok(true)
                        }
                    }
                false
            }
            IncludeAdministrative::Never => false,
        })
    }
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
