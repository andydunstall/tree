use std::path::Path;

use tree::{
    Args, DirectoriesOnlyRule, Formatter, HideHiddenRule, PriorityRule, Result, Rule, StdoutUI,
    Tree, OSFS,
};

fn rule(args: &Args) -> impl Rule {
    let mut rules: Vec<Box<dyn Rule>> = vec![];
    if !args.show_hidden {
        rules.push(Box::new(HideHiddenRule::new()));
    }
    if args.directories_only {
        rules.push(Box::new(DirectoriesOnlyRule::new()));
    }
    /*
    if args.gitignore {
        let gitignore = Gitignore::open()?;
        rules.extend(gitignore.rules());
    }
    */
    PriorityRule::new(rules)
}

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut tree = Tree::new(rule(&args), OSFS::new(), StdoutUI::new(Formatter::new()));
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
