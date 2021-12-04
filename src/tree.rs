use std::path::{Path, PathBuf};

use crate::error::Result;
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

        if let Ok(s) = self.rename(root, depth, is_last) {
            summary.add(&s);
        } else {
            let file = File::new(root, 0, 0);
            self.ui.invalid_file(file, depth, is_last, true);
        }

        self.ui.remove_dir(depth);

        summary
    }

    fn rename(&mut self, root: &Path, depth: usize, is_last: bool) -> Result<Summary> {
        let mut summary = Summary::new();

        if root.is_dir() && is_last {
            self.ui.remove_dir(depth - 1);
        }

        summary.incr(root.is_dir());
        self.ui
            .file(self.fs.metadata(root)?, depth, is_last, root.is_dir());

        let list = self.list_dir_matches(root)?;
        for i in 0..list.len() {
            let path = &list[i];
            summary.add(&self.walk(&path, depth + 1, i == list.len() - 1));
        }

        Ok(summary)
    }

    fn list_dir_matches(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for path in self.fs.list_dir(dir)? {
            if !self.filter.is_ignored(&path) {
                paths.push(path);
            }
        }
        Ok(paths)
    }
}
