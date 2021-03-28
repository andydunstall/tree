use std::path::Path;

use tree::{AnyRuleset, Args, ConfigRuleset, Formatter, Result, StdoutUI, Tree, OSFS};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let rs = AnyRuleset::new(vec![Box::new(ConfigRuleset::new(
        args.show_hidden,
        args.directories_only,
    ))]);

    let mut tree = Tree::new(rs, OSFS::new(), StdoutUI::new(Formatter::new()));
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
