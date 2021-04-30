use clap::Clap;

#[derive(Clap)]
#[clap(version = crate_version!(), author = crate_authors!(), after_help = include_str!("after_help.txt"))]
pub struct Opts {
    /// Deduplicates the path
    #[clap(short, long)]
    pub dedup: bool,
    /// Outputs line by line instead of the default colon seperated list
    #[clap(short, long)]
    pub lines: bool,
    /// Includes path's from $PATH in environment
    #[clap(short='e', long)]
    pub from_env: bool,
    /// Searches included path's using inbuild configuration
    #[clap(short, long)]
    pub included: bool,
    /// Uses the specific configuration file
    #[clap(short, long)]
    pub config: Option<String>,
    /// Use recommended flags -dei. If -e, i or -c are not set, default is assumed.
    ///
    /// Use this flag to use the recommended settings for pathfix.
    /// If no required source of paths is given, default is assumed.
    /// Usually you don't need another configuration and adding
    /// 'export PATH=$(/usr/bin/pathfix)' to your .bashrc/.zshrc/... file is enough.
    #[clap(short='D', long)]
    pub defaults: bool,
}

impl Opts {
    pub fn dedup(&self) -> bool {
        self.dedup || self.defaults()
    }

    pub fn from_env(&self) -> bool {
        self.from_env || self.defaults()
    }

    pub fn included(&self) -> bool {
        self.included || self.defaults()
    }

    pub fn defaults(&self) -> bool {
        self.defaults || (!self.from_env && !self.included && self.config.is_none())
    }
}

pub fn opts() -> Opts {
    // include Cargo.toml so a rebuild is triggered if metadata is changed
    include_str!("../../../Cargo.toml");

    Opts::parse()
}
