use std::path::Path;

use crate::entry::Entry;
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
        self.ui.file(&Entry {
            file_name: dir.to_str().unwrap().to_string(),
            depth: 0,
            is_last: false,
        });
        self.walk_nested(dir, 1)
    }

    fn walk_nested(&self, dir: &Path, depth: usize) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        let mut prev: Option<Entry> = None;
        for path in self.fs.list_dir(dir)? {
            if !self.matcher.is_match(&path) {
                continue;
            }

            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if let Some(ref entry) = prev {
                        self.ui.file(&entry);
                        prev = None;
                    }

                    if path.is_dir() {
                        self.ui.file(&Entry {
                            file_name: file_name.to_string(),
                            depth: depth,
                            is_last: false,
                        });

                        self.walk_nested(&path, depth + 1)?;
                    } else {
                        prev = Some(Entry {
                            file_name: file_name.to_string(),
                            depth: depth,
                            is_last: false,
                        });
                    }
                }
            }
        }

        if let Some(ref entry) = prev {
            self.ui.file(&Entry {
                file_name: entry.file_name.clone(),
                depth: entry.depth,
                is_last: true,
            });
        }

        Ok(())
    }
}
