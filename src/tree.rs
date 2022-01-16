use std::path::{Path, PathBuf};

use crate::filter::Filter;
use crate::fs::{File, FS};
use crate::summary::Summary;
use crate::ui::UI;

pub struct Tree<R, F, U> {
    filter: R,
    fs: F,
    ui: U,
}

impl<R, F, U> Tree<R, F, U>
where
    R: Filter,
    F: FS,
    U: UI,
{
    pub fn new(filter: R, fs: F, ui: U) -> Tree<R, F, U> {
        Tree {
            filter: filter,
            fs: fs,
            ui: ui,
        }
    }

    // list recursively walk the file tree starting at root printing each
    // element seen.
    pub fn list(&mut self, root: &Path) {
        let summary = self.walk(root, 0, false);
        self.ui.summary(&summary);
    }

    fn walk(&mut self, root: &Path, depth: usize, is_last: bool) -> Summary {
        let mut summary = Summary::new();

        self.ui.add_dir(depth);

        if root.is_dir() && is_last {
            self.ui.remove_dir(depth - 1);
        }

        let file = self.fs.open(root);
        summary.incr(&file);
        self.ui.file(&file, depth, is_last);

        if let File::Directory(dir) = file {
            let filtered = self.filter_paths(dir.contents);
            for i in 0..filtered.len() {
                let path = &filtered[i];
                if !self.filter.is_ignored(path) {
                    summary.add(&self.walk(&path, depth + 1, i == filtered.len() - 1));
                }
            }
        }

        self.ui.remove_dir(depth);

        summary
    }

    fn filter_paths(&self, paths: Vec<PathBuf>) -> Vec<PathBuf> {
        let mut filtered = vec![];
        for path in paths {
            if !self.filter.is_ignored(&path) {
                filtered.push(path);
            }
        }
        filtered
    }
}
