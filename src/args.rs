use std::path::Path;
use std::vec::Vec;

use clap::{App, Arg, ArgMatches};

use crate::error::Result;
use crate::filter::{DirectoriesOnlyFilter, Filter, HideHiddenFilter, PathFilter, PriorityFilter};
use crate::formatter::Formatter;
use crate::gitignore::open_gitignores;

#[derive(Clone, Debug)]
pub struct Args {
    // Directory to list contents of.
    root: String,
    // true if should list hidden files as well a non-hidden, false otherwise.
    show_hidden: bool,
    // true if only directorie should be listed, false otherwise.
    directories_only: bool,
    // A list of paths to ignore.
    ignore_paths: Vec<String>,
    // true if the workpace gitignore should be used to filter output, false
    // otherwise.
    filter_gitignore: bool,
    // true if the output should be displayed in long format, false otherwise.
    longformat: bool,
    // true if the output should be displayed with the number of lines per
    // file, false otherwise.
    count_lines: bool,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.4.0")
            .about("List the contents of directories in a tree-like format.")
            .arg(Arg::new("root").help("Directory to list").takes_value(true))
            .arg(Arg::new("all").short('a').help("Show hidden files"))
            .arg(
                Arg::new("ignore")
                    .short('I')
                    .long("ignore")
                    .multiple_occurrences(true)
                    .help("Path to ignore")
                    .takes_value(true),
            )
            .arg(
                Arg::new("directories")
                    .short('d')
                    .help("List directories only"),
            )
            .arg(
                Arg::new("gitignore")
                    .short('g')
                    .long("gitignore")
                    .help("Include files listed in the workspace gitignores"),
            )
            .arg(
                Arg::new("long")
                    .short('l')
                    .help("Display listing in long format"),
            )
            .arg(
                Arg::new("count")
                    .short('c')
                    .long("count")
                    .help("Display number of lines in each file"),
            )
            .get_matches();
        Ok(Args {
            root: Args::root(&matches),
            show_hidden: Args::is_enabled(&matches, "all"),
            directories_only: Args::is_enabled(&matches, "directories"),
            ignore_paths: Args::ignore_paths(&matches),
            filter_gitignore: !Args::is_enabled(&matches, "gitignore"),
            longformat: Args::is_enabled(&matches, "long"),
            count_lines: Args::is_enabled(&matches, "count"),
        })
    }

    pub fn root_path(&self) -> &Path {
        Path::new(&self.root)
    }

    // Creates a filter following the configuration in args.
    pub fn to_filter(&self) -> impl Filter {
        let mut filters: Vec<Box<dyn Filter>> = vec![];
        if !self.show_hidden {
            filters.push(Box::new(HideHiddenFilter::new()));
        }
        if self.directories_only {
            filters.push(Box::new(DirectoriesOnlyFilter::new()));
        }
        for path in &self.ignore_paths {
            filters.push(Box::new(PathFilter::new(Path::new(path))));
        }
        if self.filter_gitignore {
            for path in open_gitignores(Path::new(&self.root)) {
                // Note order important (higher priority first).
                filters.push(Box::new(path.filter()));
            }
        }
        PriorityFilter::new(filters)
    }

    pub fn to_formatter(&self) -> Formatter {
        Formatter::new(self.longformat, self.count_lines)
    }

    fn root(matches: &ArgMatches) -> String {
        matches.value_of("root").unwrap_or(".").to_string()
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
