use std::path::{Path, PathBuf};

use crate::entry::Entry;
use crate::error::Result;
use crate::formatter::Formatter;
use crate::fs::FS;
use crate::matcher::Matcher;

pub struct Tree<F> {
    matcher: Matcher,
    fs: F,
    formatter: Formatter,
}

impl<F> Tree<F>
where
    F: FS,
{
    pub fn new(matcher: Matcher, fs: F, formatter: Formatter) -> Tree<F> {
        Tree {
            matcher: matcher,
            fs: fs,
            formatter: formatter,
        }
    }

    pub fn walk(&mut self, dir: &Path) -> Result<()> {
        println!(
            "{}",
            self.formatter.file(&Entry {
                file_name: dir.to_str().unwrap().to_string(),
                depth: 0,
                is_last: false,
            })
        );
        let (n_dirs, n_files) = self.walk_nested(dir, 1)?;
        println!("{}", self.formatter.summary(n_dirs, n_files));
        Ok(())
    }

    fn walk_nested(&mut self, dir: &Path, depth: usize) -> Result<(usize, usize)> {
        if !dir.is_dir() {
            return Ok((0, 0));
        }

        self.formatter.add_indent(depth - 1);

        let mut n_dirs = 0;
        let mut n_files = 0;
        let list = self.list_dir_matches(dir)?;
        for i in 0..list.len() {
            let path = &list[i];
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if path.is_dir() {
                        if i == list.len() - 1 {
                            self.formatter.remove_indent(depth - 1);
                        }

                        n_dirs += 1;
                        println!(
                            "{}",
                            self.formatter.file(&Entry {
                                file_name: file_name.to_string(),
                                depth: depth,
                                is_last: i == list.len() - 1,
                            })
                        );

                        let (n_nested_dirs, n_nested_files) = self.walk_nested(&path, depth + 1)?;
                        n_dirs += n_nested_dirs;
                        n_files += n_nested_files;
                    } else {
                        n_files += 1;
                        println!(
                            "{}",
                            self.formatter.file(&Entry {
                                file_name: file_name.to_string(),
                                depth: depth,
                                is_last: i == list.len() - 1,
                            })
                        );
                    }
                }
            }
        }
        self.formatter.remove_indent(depth);

        Ok((n_dirs, n_files))
    }

    fn list_dir_matches(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for path in self.fs.list_dir(dir)? {
            if self.matcher.is_match(&path) {
                paths.push(path);
            }
        }
        Ok(paths)
    }
}
