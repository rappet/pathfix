#[macro_use]
extern crate clap;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate log;

#[cfg(feature = "logging")]
extern crate env_logger;

use std::collections::HashSet;

mod cli;
mod config;

use cli::{CliConfig, Mode};
use config::Config;
use std::borrow::Borrow;
use crate::config::{Path, Paths};
use std::io;

fn run(cli: &CliConfig) -> Result<(), String> {
    let mut env_config = Config::new().with_env();

    // Use paths from environment if -e is set
    if cli.from_env {
        env_config.paths = Paths::from_env().map_err(|_| "$PATH does not contain valid utf8")?;
        info!("Loaded contents of $PATH variable");
    }

    let mut config = env_config;

    if cli.included {
        let load_config = |path: &str| {
            Config::from_file(path)
                .map(|config| Some(config))
                .or_else(|err| match err.kind() {
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(err)
                })
                .map_err(|err| err.to_string())
        };

        if let Some(home_config) = load_config(
            &format!("{}/.pathfix.toml", std::env::var("HOME").map_err(|err| err.to_string())?)
        )? {
            config = config.merge(home_config);
            info!("Loaded config from ~/.pathfix.toml");
        } else {
            info!("~/.pathfix.toml is missing")
        }

        if !config.base {
            if let Some(etc_config) = load_config("/etc/pathfix.toml")? {
                config = config.merge(etc_config);
                info!("Loaded config from /etc/pathfix.toml");
            } else {
                info!("/etc/pathfix.toml is missing")
            }
        }

        // Use included paths if -s is set
        if cli.included && !config.base {
            let included_config = Config::included();

            // Merge included config and config from path
            config = config.merge(included_config);
            info!("Loaded included config")
        };
    }

    let mut path: Vec<String> = config.paths.iter()
        .map(|p| Path::from(p.clone()))
        .filter_map(|p| p.resolve(&config.env))
        .collect();

    if cli.dedup {
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

    // print output to stdout
    match cli.mode {
        Mode::Colon => {
            println!("{}", path.join(":"));
        }
        Mode::Lines => {
            for p in path {
                println!("{}", p);
            }
        }
    }

    Ok(())
}

fn main() {
    #[cfg(feature = "logging")]
        {
            env_logger::init();
        }

    let matches = cli::matches();

    let config: CliConfig = matches.borrow().into();

    std::process::exit(match run(&config) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("pathfix: {}", err);
            eprintln!("pathfix: Failure. Returning included failsave PATH");
            println!("/usr/sbin:/usr/bin:/sbin:/bin");
            1
        }
    });
}
