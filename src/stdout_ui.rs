use crate::UI;

pub struct StdoutUI;

impl StdoutUI {
    pub fn new() -> StdoutUI {
        StdoutUI {}
    }
}

impl UI for StdoutUI {
    fn file(&self, name: &str, depth: usize, is_last: bool) {
        let indent = depth - 1;
        let spacing = " ".repeat(indent * 4);
        let prefix = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", spacing, prefix, name);
    }
}
