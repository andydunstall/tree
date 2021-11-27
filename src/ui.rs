use mockall::automock;

use crate::fs::File;
use crate::summary::Summary;

#[automock]
pub trait UI {
    fn file(&self, file: File, depth: usize, is_last: bool, is_dir: bool);
    fn summary(&self, summary: &Summary);
    fn add_dir(&mut self, depth: usize);
    fn remove_dir(&mut self, depth: usize);
}
