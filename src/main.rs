use std::path::Path;

use tree::{Args, Result, StdoutUI, Tree, OSFS};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut tree = Tree::new(
        args.to_filter(),
        OSFS::new(),
        StdoutUI::new(args.to_formatter()),
    );
    tree.list(Path::new(&args.root));
    Ok(())
}
