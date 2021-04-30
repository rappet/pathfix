#[macro_use]
extern crate clap;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate log;

#[cfg(feature = "logging")]
extern crate env_logger;

use log::Level::Debug;

use std::{io, fs};
use std::collections::HashSet;

use pathfix::config::{Config, IncludeAdministrative, Paths, PathFlags};

mod cli;

mod error;
use error::Result;

fn run() -> Result<()> {
    let opts = cli::opts();

    let mut env_config = Config::new().with_env();

    // Use paths from environment if -e is set
    if opts.from_env() {
        env_config.paths = Paths::from_env()?;
        info!("Loaded contents of $PATH variable");
    }

    let mut config = env_config;

    if opts.included() {
        let load_config = |path: &str| {
            Config::from_file(path)
                .map(Some)
                .or_else(|err| match err.kind() {
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(err)
                })
        };

        if let Some(home_config) = load_config(
            &format!("{}/.pathfix.toml", std::env::var("HOME")?)
        )? {
            config = home_config.merge(config);
            info!("Loaded config from ~/.pathfix.toml");
        } else {
            info!("~/.pathfix.toml is missing")
        }

        if !config.base {
            if let Some(etc_config) = load_config("/etc/pathfix.toml")? {
                config = etc_config.merge(config);
                info!("Loaded config from /etc/pathfix.toml");
            } else {
                info!("/etc/pathfix.toml is missing")
            }
        }

        // Use included paths if -s is set
        if opts.included() && !config.base {
            let included_config = Config::included();

            // Merge included config and config from path
            config = included_config.merge(config);
            info!("Loaded included config");
        }
    }

    if let Some(config_file) = &opts.config {
        let load_config = Config::from_file(&config_file)?;
        config = load_config.merge(config);
        info!("Loaded specified config file {}", config_file);
    }

    debug!("Merged config:");
    if log_enabled!(Debug) {
        for path in config.paths.0.iter() {
            if let Some(path_source) = path.source() {
                debug!("{:30} | {:15} | {:30}", path.path(), path.flags().to_string(), path_source);
            } else {
                debug!("{:30} | {:15} |", path.path(), path.flags().to_string());
            }
        }
    }

    let include_administrative = config.include_administrative.as_ref()
        .unwrap_or(&IncludeAdministrative::RootOnly);

    let path_flags = PathFlags::this_system(include_administrative);

    let mut path = config.paths.resolve(path_flags, &config.env);

    if opts.dedup() {
        // removes duplicates while preserving order
        let (new_path, _set) = path.iter().fold(
            (Vec::new(), HashSet::new()),
            |(mut path, mut set), p| {
                if !set.contains(p) {
                    set.insert(p);
                    path.push(p.clone());
                }
                (path, set)
            });
        path = new_path;
    }

    // filter not existing paths
    path = path.into_iter().filter(
        |path| fs::metadata(path).map(
            |path| path.is_dir())
            .unwrap_or(false)
    ).collect();

    debug!("IncludeAdministrative: {:?}", config.include_administrative.clone().unwrap_or_default());
    debug!("Use admin paths: {:?}", config.include_administrative.clone().unwrap_or_default().check_current_user());

    // print output to stdout
    if opts.lines {
        for p in path {
            println!("{}", p);
        }
    } else {
        println!("{}", path.join(":"));
    }

    Ok(())
}

fn main() {
    #[cfg(feature = "logging")]
        {
            env_logger::init();
        }

    std::process::exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("pathfix: {}", err);
            eprintln!("pathfix: Failure. Returning included failsave PATH");
            println!("/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");
            1
        }
    });
}
