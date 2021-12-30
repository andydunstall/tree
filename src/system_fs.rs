use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::error::{Error, Result};
use crate::fs::{Directory, File, RegularFile, Symlink, FS};

pub struct SystemFS;

impl SystemFS {
    pub fn new() -> SystemFS {
        SystemFS {}
    }
}

impl FS for SystemFS {
    fn open(&self, path: &Path) -> File {
        if is_symlink(path) {
            if let Ok(symlink) = open_symlink(path) {
                File::Symlink(symlink)
            } else {
                File::Symlink(Symlink {
                    name: path_to_filename(path),
                    target: "?".to_string(),
                    accessible: false,
                })
            }
        } else if path.is_dir() {
            if let Ok(dir) = open_directory(path) {
                File::Directory(dir)
            } else {
                File::Directory(Directory {
                    name: path_to_filename(path),
                    contents: vec![],
                    accessible: false,
                })
            }
        } else {
            if let Ok(file) = open_regular_file(path) {
                File::RegularFile(file)
            } else {
                File::RegularFile(RegularFile {
                    name: path_to_filename(path),
                    size: 0,
                    line_count: 0,
                    executable: false,
                    accessible: false,
                })
            }
        }
    }
}

fn open_regular_file(path: &Path) -> Result<RegularFile> {
    Ok(RegularFile {
        name: path_to_filename(path),
        size: file_size(path)?,
        line_count: line_count(path)?,
        executable: is_executable(path)?,
        accessible: true,
    })
}

fn open_directory(path: &Path) -> Result<Directory> {
    Ok(Directory {
        name: path_to_filename(path),
        contents: list_dir(path)?,
        accessible: true,
    })
}

fn open_symlink(path: &Path) -> Result<Symlink> {
    let target = fs::read_link(path)?
        .into_os_string()
        .to_str()
        .ok_or(Error::new("nvalid symlink"))?
        .to_string();
    Ok(Symlink {
        name: path_to_filename(path),
        target,
        accessible: path.exists(),
    })
}

fn file_size(path: &Path) -> Result<u64> {
    if path.is_dir() {
        Ok(0)
    } else {
        Ok(fs::metadata(path)?.len())
    }
}

fn is_executable(path: &Path) -> Result<bool> {
    let mode = fs::metadata(path)?.mode();
    Ok(mode & 0o111 != 0)
}

fn line_count(path: &Path) -> Result<u64> {
    if path.is_dir() {
        return Ok(0);
    }
    let f = fs::File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 32, f);
    let mut count = 0;
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            if buf.is_empty() {
                break;
            }
            count += bytecount::count(&buf, b'\n');
            buf.len()
        };
        reader.consume(len);
    }
    Ok(count as u64)
}

fn list_dir(dir: &Path) -> Result<Vec<PathBuf>> {
    if !dir.is_dir() {
        return Ok(vec![]);
    }

    let mut paths = vec![];
    for entry in fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}

fn is_symlink(path: &Path) -> bool {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        metadata.file_type().is_symlink()
    } else {
        false
    }
}

fn path_to_filename(path: &Path) -> String {
    if let Some(file_name) = path.file_name() {
        // Assume unicode path.
        file_name.to_str().unwrap().to_string()
    } else {
        ".".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::os::unix::fs::symlink;
    use std::os::unix::fs::OpenOptionsExt;

    use rand::{distributions::Alphanumeric, Rng};
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn open_regular_file() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let path = dir.path().join(name.clone());

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o660)
            .open(path.clone())
            .unwrap();
        file.write_all(b"a\nb\nc\n").unwrap();

        let expected = File::RegularFile(RegularFile {
            name,
            size: 6,
            line_count: 3,
            executable: false,
            accessible: true,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path));
    }

    #[test]
    fn open_regular_file_executable() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let path = dir.path().join(name.clone());

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o770)
            .open(path.clone())
            .unwrap();
        file.write_all(b"a\nb\nc\n").unwrap();

        let expected = File::RegularFile(RegularFile {
            name,
            size: 6,
            line_count: 3,
            executable: true,
            accessible: true,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path));
    }

    #[test]
    fn open_regular_file_inaccessible() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let path_not_found = dir.path().join(name.clone());

        let expected = File::RegularFile(RegularFile {
            name,
            size: 0,
            line_count: 0,
            executable: false,
            accessible: false,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path_not_found));
    }

    #[test]
    fn open_directory() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let path = dir.path().join(name.clone());

        fs::create_dir(path.clone()).unwrap();

        let expected = File::Directory(Directory {
            name,
            contents: vec![],
            accessible: true,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path));
    }

    #[test]
    fn open_symlink() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let target = random_path();
        let path = dir.path().join(name.clone());

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o660)
            .open(dir.path().join(target.clone()))
            .unwrap();
        file.write_all(b"a\nb\nc\n").unwrap();

        symlink(target.clone(), path.clone()).unwrap();

        let expected = File::Symlink(Symlink {
            name,
            target,
            accessible: true,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path));
    }

    #[test]
    fn open_symlink_inaccessible() {
        let dir = tempdir().unwrap();
        let name = random_path();
        let target = random_path();
        let path = dir.path().join(name.clone());

        // Point symlink to target which does not exist.
        symlink(target.clone(), path.clone()).unwrap();

        let expected = File::Symlink(Symlink {
            name,
            target,
            accessible: false,
        });

        let fs = SystemFS::new();
        assert_eq!(expected, fs.open(&path));
    }

    fn random_path() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
    }
}
