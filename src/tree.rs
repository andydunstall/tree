use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::fs::FS;
use crate::rule::Rule;
use crate::summary::Summary;
use crate::ui::UI;

pub struct Tree<R, F, U> {
    rule: R,
    fs: F,
    ui: U,
}

impl<R, F, U> Tree<R, F, U>
where
    R: Rule,
    F: FS,
    U: UI,
{
    pub fn new(rule: R, fs: F, ui: U) -> Tree<R, F, U> {
        Tree {
            rule: rule,
            fs: fs,
            ui: ui,
        }
    }

    pub fn walk(&mut self, dir: &Path) -> Result<()> {
        self.ui
            .file(dir.to_str().unwrap().to_string(), 0, 0, 0, false, true);
        let summary = self.walk_nested(dir, 1)?;
        self.ui.summary(&summary);
        Ok(())
    }

    // TODO(AD) Refactor and TDD
    fn walk_nested(&mut self, dir: &Path, depth: usize) -> Result<Summary> {
        let mut summary = Summary {
            n_dirs: 0,
            n_files: 0,
        };

        self.ui.add_dir(depth - 1);

        let list = self.list_dir_matches(dir)?;
        for i in 0..list.len() {
            let path = &list[i];
            if let Some(file_name) = path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    if path.is_dir() {
                        if i == list.len() - 1 {
                            self.ui.remove_dir(depth - 1);
                        }

                        summary.n_dirs += 1;
                        self.ui.file(
                            file_name.to_string(),
                            0,
                            0,
                            depth,
                            i == list.len() - 1,
                            true,
                        );

                        summary.add(&self.walk_nested(&path, depth + 1)?);
                    } else {
                        summary.n_files += 1;
                        self.ui.file(
                            file_name.to_string(),
                            self.fs.file_size(path)?,
                            self.fs.line_count(path)?,
                            depth,
                            i == list.len() - 1,
                            false,
                        );
                    }
                }
            }
        }
        self.ui.remove_dir(depth);

        Ok(summary)
    }

    fn list_dir_matches(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for path in self.fs.list_dir(dir)? {
            if !self.rule.is_ignored(&path) {
                paths.push(path);
            }
        }
        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockall::predicate;

    use crate::fs::MockFS;
    use crate::rule::MockRule;
    use crate::ui::MockUI;

    #[test]
    fn dir_does_not_exist() {
        let rule = MockRule::new();
        let mut fs = MockFS::new();
        fs.expect_list_dir()
            .with(predicate::eq(Path::new("mydir")))
            .times(1)
            .returning(|_| Ok(vec![]));

        let mut ui = MockUI::new();
        ui.expect_file()
            .with(
                predicate::eq("mydir".to_string()),
                predicate::eq(0),
                predicate::eq(0),
                predicate::eq(0),
                predicate::eq(false),
                predicate::eq(true),
            )
            .times(1)
            .returning(|_, _, _, _, _, _| ());
        ui.expect_add_dir()
            .with(predicate::eq(0))
            .times(1)
            .returning(|_| ());
        ui.expect_remove_dir()
            .with(predicate::eq(1))
            .times(1)
            .returning(|_| ());
        ui.expect_summary()
            .with(predicate::eq(&Summary {
                n_dirs: 0,
                n_files: 0,
            }))
            .times(1)
            .returning(|_| ());

        let mut tree = Tree::new(rule, fs, ui);
        tree.walk(Path::new("mydir")).unwrap();
    }
}
