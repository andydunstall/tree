use std::path::Path;

use crate::error::Result;
use crate::fs::FS;
use crate::matcher::Matcher;
use crate::ui::UI;

pub struct Tree<F, U> {
    matcher: Matcher,
    fs: F,
    ui: U,
}

impl<F, U> Tree<F, U>
where
    F: FS,
    U: UI,
{
    pub fn new(matcher: Matcher, fs: F, ui: U) -> Tree<F, U> {
        Tree {
            matcher: matcher,
            fs: fs,
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

        for path in self.fs.list_dir(dir)? {
            let path = path?;
            if !self.matcher.is_match(&path) {
                continue;
            }

            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    self.ui.file(file_name, depth, false);
                    if path.is_dir() {
                        self.walk_nested(&path, depth + 1)?;
                    }
                }
            }
        }

        Ok(())
    }
}
