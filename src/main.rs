#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use std::collections::HashSet;

struct Config {
    pub dedup: bool,
    pub mode: Mode,
}

impl Config {
    fn from_matches(matches: &ArgMatches) -> Config {
        let mode = if matches.is_present("lines") {
            Mode::Lines
        } else {
            Mode::Colon
        };

        Config {
            dedup: !matches.is_present("norecommended") || matches.is_present("dedup"),
            mode
        }
    }
}

enum Mode {
    Colon, Lines,
}

fn run(config: &Config) -> Result<(), &'static str> {
    let path_str = std::env::var("PATH").or(Err("Could not geth $PATH environment variable"))?;
    let mut path: Vec<&str> = path_str.split(':').collect();

    if config.dedup {
        // removes duplicates while preserving order
        let (new_path, _set) = path.iter().fold(
            (Vec::new(), HashSet::new()),
            |(mut path, mut set), p| {
                if !set.contains(*p) {
                    set.insert(*p);
                    path.push(*p);
                }
                (path, set)
            });
        path = new_path;
    }
    
    // print output to stdout
    match config.mode {
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
    let config = Config::from_matches(&matches);

    std::process::exit(match run(&config) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}