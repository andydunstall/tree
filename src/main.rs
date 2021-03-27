use std::path::Path;

use tree::{Args, CLIFormatter, Matcher, Result, Tree, OSFS};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;
    let matcher = Matcher::new(args.show_hidden, args.directories_only);
    let mut tree = Tree::new(matcher, OSFS::new(), CLIFormatter::new());
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
