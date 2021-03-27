use clap::{App, Arg, ArgMatches};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Args {
    pub dir: String,
    pub show_hidden: bool,
    pub directories_only: bool,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.1.0")
            .about("List the contents of directories in a tree-like format.")
            .arg(
                Arg::with_name("directory")
                    .help("Directory to list")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("directories")
                    .short("d")
                    .help("List directories only"),
            )
            .arg(Arg::with_name("all").short("a").help("Show hidden files"))
            .get_matches();
        Ok(Args {
            dir: Args::dir(&matches),
            show_hidden: Args::is_enabled(&matches, "all"),
            directories_only: Args::is_enabled(&matches, "directories"),
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
