use std::path::Path;

use tree::{
    open_gitignores, Args, DirectoriesOnlyFilter, Filter, Formatter, HideHiddenFilter, PathFilter,
    PriorityFilter, Result, StdoutUI, Tree, OSFS,
};

// Creates a filter following the configuration in args.
// TODO(AD) Rename filter to filter.
fn filter_from_args(args: &Args) -> impl Filter {
    let mut filters: Vec<Box<dyn Filter>> = vec![];
    if !args.show_hidden {
        filters.push(Box::new(HideHiddenFilter::new()));
    }
    if args.directories_only {
        filters.push(Box::new(DirectoriesOnlyFilter::new()));
    }
    for path in &args.ignore_paths {
        filters.push(Box::new(PathFilter::new(Path::new(path))));
    }
    if args.filter_gitignore {
        for path in open_gitignores(Path::new(&args.dir)) {
            // Note order important (higher priority first).
            filters.push(Box::new(path.filter()));
        }
    }
    PriorityFilter::new(filters)
}

fn main() -> Result<()> {
    let args = Args::parse_cli()?;

    let mut tree = Tree::new(
        filter_from_args(&args),
        OSFS::new(),
        StdoutUI::new(Formatter::new(args.longformat, args.count_lines)),
    );
    // TODO(AD) tree.list
    tree.walk(Path::new(&args.dir))?;
    Ok(())
}
