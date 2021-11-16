use std::path::Path;

use tree::{
    open_gitignores, open_treeignore, Args, DirectoriesOnlyRule, Formatter, HideHiddenRule,
    PathRule, PriorityRule, Result, Rule, StdoutUI, Tree, OSFS,
};

fn rule(args: &Args) -> impl Rule {
    let mut rules: Vec<Box<dyn Rule>> = vec![];
    if !args.show_hidden {
        rules.push(Box::new(HideHiddenRule::new()));
    }
    if args.directories_only {
        rules.push(Box::new(DirectoriesOnlyRule::new()));
    }
    for ignore in &args.ignore {
        rules.push(Box::new(PathRule::new(Path::new(ignore))));
    }
    if let Some(treeignore) = open_treeignore() {
        rules.push(Box::new(treeignore.rule()));
    }
    if args.gitignore {
        for gitignore in open_gitignores(Path::new(&args.dir)) {
            // Note order important (higher priority first).
            rules.push(Box::new(gitignore.rule()));
        }
    }
    PriorityRule::new(rules)
}

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut tree = Tree::new(
        rule(&args),
        OSFS::new(),
        StdoutUI::new(Formatter::new(args.longformat, args.count_lines)),
    );
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
