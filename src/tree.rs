use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::matcher::Matcher;
use crate::ui::UI;

pub struct Tree<U> {
    matcher: Matcher,
    ui: U,
}

impl<U> Tree<U>
where
    U: UI,
{
    pub fn new(matcher: Matcher, ui: U) -> Tree<U> {
        Tree {
            matcher: matcher,
            ui: ui,
        }
    }

    pub fn walk(&self, dir: &Path) -> Result<()> {
        self.walk_nested(dir, 1)
    }

    fn walk_nested(&self, dir: &Path, depth: usize) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        // TODO(AD) Only count those that match (not hidden)
        let count = fs::read_dir(dir)?.count();
        let mut n = 0;
        for entry in fs::read_dir(dir)? {
            n += 1;

            let entry = entry?;
            let path = entry.path();

            if !self.matcher.is_match(&path) {
                continue;
            }

            // TODO(AD) Remvoe unwrap
            let name = path.file_name().unwrap().to_str().unwrap();
            self.ui.file(name, depth, n == count);
            if path.is_dir() {
                self.walk_nested(&path, depth + 1)?;
            }
        }

        Ok(())
    }
}
