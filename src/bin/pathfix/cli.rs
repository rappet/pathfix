use clap::{ArgMatches, Arg, ArgGroup};

pub struct CliConfig {
    pub dedup: bool,
    pub mode: Mode,
    pub from_env: bool,
    pub included: bool,
}

pub fn matches() -> ArgMatches<'static> {
    // include Cargo.toml so a rebuild is triggered if metadata is changed
    include_str!("../../../Cargo.toml");

    clap::app_from_crate!()
        .after_help(include_str!("after_help.txt"))
        .args_from_usage(
            "-d, --dedup 'Deduplicates the path'
                    -l, --lines 'Outputs line by line instead of the default colon seperated list'
                    -e, --from-env 'Includes path's from $PATH in environment'
                    -i, --included 'Searches included path's using inbuild configuration'
            "
        )
        .arg(Arg::from_usage("-D, --defaults 'Use recommended flags -des. Either -D, -e or -s must be set'")
            .long_help(
"Use this flag to use the recommended settings for pathfix.
Usually you don't need another configuration and adding
'export PATH=$(/usr/bin/pathfix -D)' to your .bashrc/.zshrc/... file is enough. "))
        .group(ArgGroup::with_name("source")
            .args(&["from-env", "included", "defaults"])
            .required(true)
            .multiple(true))
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
            from_env: recommended || matches.is_present("from-env"),
            included: recommended || matches.is_present("included"),
        }
    }
}

pub enum Mode {
    Colon,
    Lines,
}
