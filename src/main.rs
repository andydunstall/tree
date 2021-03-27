use std::path::Path;

use tree::{Args, Result, Tree};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let tree = Tree::new();
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
