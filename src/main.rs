#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_yaml;

use clap::App;

use std::collections::HashSet;

mod cli;
mod config;

use cli::{CliConfig, Mode};
use config::Config;
use std::borrow::Borrow;
use crate::config::Path;

fn run(cli: &CliConfig) -> Result<(), &'static str> {
    let mut env_config = Config::new().with_env();

    // Use paths from environment if -e is set
    if cli.from_env {
        env_config = env_config.with_path_from_env()?;
    }

    // Use included paths if -s is set
    let included_config = if cli.with_searches {
        Config::included()
    } else {
        Config::new()
    };

    // Merge included config and config from path
    let config = env_config.merge(included_config);

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
        },
        Mode::Lines => {
            for p in path {
                println!("{}", p);
            }
        }
    }

    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config: CliConfig = matches.borrow().into();

    std::process::exit(match run(&config) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
