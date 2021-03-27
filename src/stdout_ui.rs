use crate::entry::Entry;
use crate::ui::UI;

pub struct StdoutUI;

impl StdoutUI {
    pub fn new() -> StdoutUI {
        StdoutUI {}
    }
}

impl UI for StdoutUI {
    fn file(&self, entry: &Entry) {
        if entry.depth > 0 {
            let indent = entry.depth - 1;
            let spacing = "│   ".repeat(indent);
            let prefix = if entry.is_last {
                "└── "
            } else {
                "├── "
            };
            println!("{}{}{}", spacing, prefix, entry.file_name);
        } else {
            println!("{}", entry.file_name);
        }
    }
}
