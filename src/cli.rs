use clap::{ArgMatches};

pub struct CliConfig {
    pub dedup: bool,
    pub mode: Mode,
    pub from_env: bool,
    pub with_searches: bool,
}

pub fn matches() -> ArgMatches<'static> {
    // include Cargo.toml so a rebuild is triggered if metadata is changed
    include_str!("../Cargo.toml");

    clap::app_from_crate!()
        .args_from_usage(
            "-d, --dedup 'Deduplicates the path. Default set if -R is not set'
                    -D, --defaults 'Use recommended flags'
                    -l, --lines 'Outputs line by line instead of the default colon seperated list'
                    -e, --from-env 'Includes path's from $PATH in environment'
                    -s, --with-searched 'Searches valid path's using inbuild rules'
            "
        )
        .get_matches()
}

impl From<&ArgMatches<'_>> for CliConfig {
    fn from(matches: &ArgMatches) -> Self {
        let mode = if matches.is_present("lines") {
            Mode::Lines
        } else {
            Mode::Colon
        };

        let recommended = matches.is_present("defaults");
        CliConfig {
            dedup: recommended || matches.is_present("dedup"),
            mode,
            from_env: recommended || matches.is_present("from_env"),
            with_searches: recommended || matches.is_present("with_searches"),
        }
    }
}

pub enum Mode {
    Colon,
    Lines,
}
