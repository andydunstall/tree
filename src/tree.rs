use std::fs;
use std::path::Path;

use crate::error::Result;

pub struct Tree {}

impl Tree {
    pub fn new() -> Tree {
        Tree {}
    }

    pub fn walk(&self, dir: &Path) -> Result<()> {
        self.walk_nested(dir, 0)
    }

    fn walk_nested(&self, dir: &Path, depth: usize) -> Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let name = path.file_name().unwrap().to_str().unwrap();
            if name.starts_with(".") {
                continue;
            }

            if path.is_dir() {
                let indent = " ".repeat(depth * 2);
                println!("{}{}/", indent, name);
                self.walk_nested(&path, depth + 1)?;
            } else {
                println!("{}{}", " ".repeat(depth * 2), name);
            }
        }

        Ok(())
    }
}
