use std::vec::Vec;

use clap::{App, Arg, ArgMatches};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Args {
    pub dir: String,
    pub show_hidden: bool,
    pub directories_only: bool,
    pub gitignore: bool,
    pub treeignore: bool,
    pub ignore: Vec<String>,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.2.0")
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
            .arg(
                Arg::with_name("gitignore")
                    .short("g")
                    .long("gitignore")
                    .help("Hide files listed in the gitignore"),
            )
            .arg(
                Arg::with_name("notreeignore")
                    .short("c")
                    .help("Disable `~/.treeignore`"),
            )
            .arg(Arg::with_name("all").short("a").help("Show hidden files"))
            .arg(
                Arg::with_name("ignore")
                    .short("I")
                    .long("ignore")
                    .multiple(true)
                    .help("Path to ignore")
                    .takes_value(true),
            )
            .get_matches();
        Ok(Args {
            dir: Args::dir(&matches),
            show_hidden: Args::is_enabled(&matches, "all"),
            directories_only: Args::is_enabled(&matches, "directories"),
            gitignore: Args::is_enabled(&matches, "gitignore"),
            treeignore: !Args::is_enabled(&matches, "notreeignore"),
            ignore: Args::ignore(&matches),
        })
    }

    fn dir(matches: &ArgMatches) -> String {
        matches.value_of("directory").unwrap_or(".").to_string()
    }

    fn ignore(matches: &ArgMatches) -> Vec<String> {
        if let Some(ignore) = matches.values_of("ignore") {
            let ignore: Vec<String> = ignore.map(|s| s.to_string()).collect();
            ignore
        } else {
            vec![]
        }
    }

    fn is_enabled(matches: &ArgMatches, key: &str) -> bool {
        match matches.occurrences_of(key) {
            0 => false,
            _ => true,
        }
    }
}
