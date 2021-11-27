use std::path::{Component, Path, PathBuf};

use crate::filter::Filter;

// TODO(AD) Support glob
pub struct PathFilter {
    path: PathBuf,
}

impl PathFilter {
    pub fn new(path: &Path) -> PathFilter {
        PathFilter {
            path: normalize_path(path),
        }
    }
}

impl Filter for PathFilter {
    fn is_ignored(&self, path: &Path) -> bool {
        let mut path = normalize_path(path);
        loop {
            if path.starts_with(self.path.clone()) {
                return true;
            }

            if let Some(prefix) = path.iter().next() {
                if let Ok(trunk) = path.strip_prefix(prefix) {
                    path = trunk.to_path_buf();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    fn is_override(&self, _path: &Path) -> bool {
        false
    }
}

// From https://github.com/rust-lang/cargo/blob/fede83ccf973457de319ba6fa0e36ead454d2e20/src/cargo/util/paths.rs#L61
// as canonicalize requires the file to exist.
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(AD) Glob

    #[test]
    fn ignore_nested_path() {
        let filter = PathFilter::new(Path::new("a/b"));
        assert!(filter.is_ignored(Path::new("./a/b")));
        assert!(filter.is_ignored(Path::new("c/a/b")));
        assert!(filter.is_ignored(Path::new("c/a/b/c")));
        assert!(filter.is_ignored(Path::new("./a/./b/../b/c")));
        assert!(filter.is_ignored(Path::new("./x/y/./a/./b/../b/c")));
        assert!(!filter.is_ignored(Path::new("a/c/b")));
    }

    #[test]
    fn ignore_nested_file() {
        let filter = PathFilter::new(Path::new("myfile"));
        assert!(filter.is_ignored(Path::new("dir/myfile")));
        assert!(filter.is_ignored(Path::new("dir/./myfile")));
        assert!(filter.is_ignored(Path::new("myfile/dir")));
    }

    #[test]
    fn ignore_path() {
        let filter = PathFilter::new(Path::new("a/b/c"));
        assert!(filter.is_ignored(Path::new("a/b/c")));
        assert!(filter.is_ignored(Path::new("./a/b/c")));
        assert!(filter.is_ignored(Path::new("./a/./b/../b/c")));
        assert!(!filter.is_ignored(Path::new("a/b")));
    }

    #[test]
    fn ignore_file() {
        let filter = PathFilter::new(Path::new("myfile"));
        assert!(filter.is_ignored(Path::new("myfile")));
        assert!(filter.is_ignored(Path::new("././myfile")));
        assert!(!filter.is_ignored(Path::new("notmyfile")));
        assert!(!filter.is_ignored(Path::new("././notmyfile")));
    }
}
