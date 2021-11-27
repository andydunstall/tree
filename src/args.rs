use std::vec::Vec;

use clap::{App, Arg, ArgMatches};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Args {
    // Directory to list contents of.
    pub dir: String,
    // true if should list hidden files as well a non-hidden, false otherwise.
    pub show_hidden: bool,
    // true if only directorie should be listed, false otherwise.
    pub directories_only: bool,
    // A list of paths to ignore.
    // TODO(AD) what is this matching
    pub ignore_paths: Vec<String>,
    // true if the workpace gitignore should be used to filter output, false
    // otherwise.
    pub filter_gitignore: bool,
    // true if the output should be displayed in long format, false otherwise.
    pub longformat: bool,
    // true if the output should be displayed with the number of lines per
    // file, false otherwise.
    pub count_lines: bool,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.3.0")
            .about("List the contents of directories in a tree-like format.")
            .arg(
                Arg::with_name("directory")
                    .help("Directory to list")
                    .takes_value(true),
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
            .arg(
                Arg::with_name("directories")
                    .short("d")
                    .help("List directories only"),
            )
            .arg(
                Arg::with_name("gitignore")
                    .short("g")
                    .long("gitignore")
                    .help("Include files listed in the workspace gitignores"),
            )
            .arg(
                Arg::with_name("long")
                    .short("l")
                    .help("Display listing in long format"),
            )
            .arg(
                Arg::with_name("count")
                    .short("c")
                    .long("count")
                    .help("Display number of lines in each file"),
            )
            .get_matches();
        Ok(Args {
            dir: Args::dir(&matches),
            show_hidden: Args::is_enabled(&matches, "all"),
            directories_only: Args::is_enabled(&matches, "directories"),
            ignore_paths: Args::ignore_paths(&matches),
            filter_gitignore: !Args::is_enabled(&matches, "gitignore"),
            longformat: Args::is_enabled(&matches, "long"),
            count_lines: Args::is_enabled(&matches, "count"),
        })
    }

    fn dir(matches: &ArgMatches) -> String {
        matches.value_of("directory").unwrap_or(".").to_string()
    }

    fn ignore_paths(matches: &ArgMatches) -> Vec<String> {
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
