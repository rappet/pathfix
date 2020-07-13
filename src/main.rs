#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use std::collections::HashSet;

mod path;

use path::ConfigurablePath;

fn default_paths() -> Vec<ConfigurablePath<'static>> {
    vec![
        ConfigurablePath::with_env(".cargo/bin", "HOME", "/"),
        ConfigurablePath::with_env("bin", "GOPATH", "/usr/local/go"),
        ConfigurablePath::with_env("bin", "GOROOT", "/usr/local/go"),
        ConfigurablePath::with_env(".local/bin", "HOME", "/"),
        ConfigurablePath::with_env("bin", "HOME", "/"),
        ConfigurablePath::new("/snap"),
        ConfigurablePath::new_become("/usr/local/sbin"),
        ConfigurablePath::new("/usr/local/bin"),
        ConfigurablePath::new_become("/usr/sbin"),
        ConfigurablePath::new("/usr/bin"),
        ConfigurablePath::new_become("/sbin"),
        ConfigurablePath::new("/bin"),
        ConfigurablePath::new("/usr/local/games"),
        ConfigurablePath::new("/usr/games"),
    ]
}

struct Config {
    pub dedup: bool,
    pub mode: Mode,
    pub from_env: bool,
    pub with_searches: bool,
}

impl Config {
    fn from_matches(matches: &ArgMatches) -> Config {
        let mode = if matches.is_present("lines") {
            Mode::Lines
        } else {
            Mode::Colon
        };

        let recommended = !matches.is_present("norecommended");
        Config {
            dedup:  recommended || matches.is_present("dedup"),
            mode,
            from_env: recommended || matches.is_present("from_env"),
            with_searches: recommended || matches.is_present("with_searches"),
        }
    }
}

enum Mode {
    Colon, Lines,
}

fn run(config: &Config) -> Result<(), &'static str> {
    let path_str = std::env::var("PATH").or(Err("Could not geth $PATH environment variable"))?;
    let mut path: Vec<String> = if config.from_env {
        path_str.split(':').map(String::from).collect()
    } else {
        Vec::new()
    };

    if config.with_searches {
        let mut searches: Vec<String> = default_paths().iter()
            .filter_map(ConfigurablePath::to_std_pathbuf_checked)
            .map(|p| String::from(p.to_str().expect("Path has non utf8 characters")))
            .collect();
        path.append(&mut searches);
    }

    if config.dedup {
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
