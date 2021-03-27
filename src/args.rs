use clap::{App, Arg, ArgMatches};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct Args {
    pub dir: String,
}

impl Args {
    pub fn parse_cli() -> Result<Args> {
        let matches = App::new("tree")
            .version("0.1.0")
            .about("list contents of directories in a tree-like format")
            .arg(
                Arg::with_name("dir")
                    .short("d")
                    .long("dir")
                    .help("directory to list")
                    .takes_value(true),
            )
            .get_matches();
        Ok(Args {
            dir: Args::dir(&matches),
        })
    }

    fn dir(matches: &ArgMatches) -> String {
        matches.value_of("dir").unwrap_or(".").to_string()
    }
}
