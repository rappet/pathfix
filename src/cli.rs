use clap::ArgMatches;

pub struct CliConfig {
    pub dedup: bool,
    pub mode: Mode,
    pub from_env: bool,
    pub with_searches: bool,
}

impl From<&ArgMatches<'_>> for CliConfig {
    fn from(matches: &ArgMatches) -> Self {
        let mode = if matches.is_present("lines") {
            Mode::Lines
        } else {
            Mode::Colon
        };

        let recommended = !matches.is_present("norecommended");
        CliConfig {
            dedup:  recommended || matches.is_present("dedup"),
            mode,
            from_env: recommended || matches.is_present("from_env"),
            with_searches: recommended || matches.is_present("with_searches"),
        }
    }
}

pub enum Mode {
    Colon, Lines,
}
