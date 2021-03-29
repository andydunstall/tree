use std::fs;
use std::path::Path;

pub use crate::error::Result;
pub use crate::ignore_config::IgnoreConfig;
pub use crate::rule::{OverrideRule, PathRule, PriorityRule, Rule};

const TREEIGNORE: &str = ".treeignore";

pub fn open_treeignore() -> Option<IgnoreConfig> {
    if let Some(home) = dirs::home_dir() {
        let path = home.join(TREEIGNORE);
        if let Ok(treeignore) = fs::read_to_string(path) {
            Some(IgnoreConfig::new(&treeignore, Path::new("/")))
        } else {
            None
        }
    } else {
        None
    }
}
