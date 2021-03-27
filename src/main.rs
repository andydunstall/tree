use std::path::Path;

use tree::{Args, Matcher, Result, StdoutUI, Tree};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    // TODO(AD) Matcher params from args
    let matcher = Matcher::new(false);
    let tree = Tree::new(matcher, StdoutUI::new());
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
