use tree::{Args, Result, StdoutUI, SystemFS, Tree};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut tree = Tree::new(
        args.to_filter(),
        SystemFS::new(),
        StdoutUI::new(args.to_formatter()),
    );
    tree.list(args.root_path());

    Ok(())
}
