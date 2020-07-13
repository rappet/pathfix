use std::path::{Path, PathBuf};
use std::default::Default;
use std::ffi::OsString;

#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct ConfigurablePath<'a> {
    pub path: &'a str,
    pub base_env: Option<&'a str>,
    pub base_default: Option<&'a str>,
    pub become_only: bool,
}

impl ConfigurablePath<'_> {
    pub fn new(path: &str) -> ConfigurablePath {
        ConfigurablePath {
            path,
            ..Default::default()
        }
    }

    pub fn new_become(path: &str) -> ConfigurablePath {
        ConfigurablePath {
            path,
            become_only: true,
            ..Default::default()
        }
    }

    pub fn with_env<'a>(path: &'a str, base_env: &'a str, base_default: &'a str) -> ConfigurablePath<'a> {
        ConfigurablePath {
            path,
            base_env: Some(base_env),
            base_default: Some(base_default),
            ..Default::default()
        }
    }

    // TODO check for sudo
    pub fn to_std_pathbuf(&self) -> Option<PathBuf> {
        let mut os_path = PathBuf::from(self.path);
        let base = self.base_env
            .and_then(std::env::var_os)
            .or(self.base_default.map(OsString::from));
        if let Some(base) = base {
            os_path = Path::new(&base).join(os_path);
        }
        Some(os_path)
    }

    pub fn to_std_pathbuf_checked(&self) -> Option<PathBuf> {
        if let Some(path) = self.to_std_pathbuf() {
            if let Ok(metadata) = path.metadata() {
                if metadata.is_dir() {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::path::ConfigurablePath;

    #[test]
    fn test_default() {
        let p = ConfigurablePath {
            path: "/foo/bar",
            base_env: None,
            base_default: None,
            become_only: false
        };

        assert_eq!(p, ConfigurablePath::new("/foo/bar"))
    }
}
