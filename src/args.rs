use clap::{App, Arg, ArgMatches};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Args {
    pub dir: String,
    pub show_hidden: bool,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.1.0")
            .about("list contents of directories in a tree-like format")
            .arg(
                Arg::with_name("directory")
                    .help("directory to list")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("show hidden files"),
            )
            .get_matches();
        Ok(Args {
            dir: Args::dir(&matches),
            show_hidden: Args::is_enabled(&matches, "all"),
        })
    }

    fn dir(matches: &ArgMatches) -> String {
        matches.value_of("directory").unwrap_or(".").to_string()
    }

    fn is_enabled(matches: &ArgMatches, key: &str) -> bool {
        match matches.occurrences_of(key) {
            0 => false,
            _ => true,
        }
    }
}
