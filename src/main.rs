use std::path::Path;

use tree::{
    AnyRuleset, Args, ConfigRuleset, Formatter, GitignoreRuleset, Result, Ruleset, StdoutUI, Tree,
    OSFS,
};

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut rulesets: Vec<Box<dyn Ruleset>> = vec![Box::new(ConfigRuleset::new(
        args.show_hidden,
        args.directories_only,
    ))];
    if args.gitignore {
        for rs in GitignoreRuleset::open(Path::new(&args.dir))? {
            rulesets.push(Box::new(rs));
        }
    }

    let rs = AnyRuleset::new(vec![Box::new(ConfigRuleset::new(
        args.show_hidden,
        args.directories_only,
    ))]);

    let mut tree = Tree::new(rs, OSFS::new(), StdoutUI::new(Formatter::new()));
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
