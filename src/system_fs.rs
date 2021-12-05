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
        match entry {
            Ok(entry) => {
                paths.push(entry.path());
            }
            Err(err) => {
                return Err(Error::from(err));
            }
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
        return file_name.to_str().unwrap().to_string();
    } else {
        return ".".to_string();
    }
}
