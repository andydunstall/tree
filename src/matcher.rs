use std::path::Path;

pub struct Matcher {
    show_hidden: bool,
}

impl Matcher {
    pub fn new(show_hidden: bool) -> Matcher {
        Matcher {
            show_hidden: show_hidden,
        }
    }

    pub fn is_match(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name) = file_name.to_str() {
                self.is_file_name_match(file_name)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_file_name_match(&self, file_name: &str) -> bool {
        let hidden = file_name.starts_with(".");
        if hidden && !self.show_hidden {
            false
        } else {
            true
        }
    }
}
