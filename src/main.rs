use std::path::Path;

use tree::{Args, Matcher, Result, StdoutUI, Tree};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let matcher = Matcher::new(args.show_hidden);
    let tree = Tree::new(matcher, StdoutUI::new());
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
