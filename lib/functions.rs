use colored::*;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{self, BufRead, ErrorKind::AlreadyExists, Lines};
use std::iter::Iterator;
use std::path::{Path, PathBuf};

use super::{PathKind::*, *};

/// Returns an entry in the tree, where the first result is the depth,
/// and the second result is the file
pub fn get_entry(mut entry: &str) -> (i32, &str) {
    let mut depth = 0;

    loop {
        match either!(
            entry.strip_prefix("    "),
            entry.strip_prefix("└── "),
            entry.strip_prefix("├── "),
            entry.strip_prefix("│   "),
            // Some iplementations of tree use a non-breaking space here (ua0)
            entry.strip_prefix("│\u{a0}\u{a0} ")
        ) {
            Some(suffix) => {
                entry = suffix;
                depth += 1;
            }
            None => return (depth, entry),
        }
    }
}

/// Atomically create a file, if it doesn't already exist. This is an atomic operation on the filesystem.
/// If the file already exists, this function exits without affecting that file.
pub fn touch_file(path: &Path) -> io::Result<()> {
    // create_new is used to implement creation + existence checking as an atomic filesystem operation.

    // create_new is used instead of create because the program should NOT
    // attempt to open files that already exist. This could result in an exception being thrown
    // if the file is locked by another program, or marked as read only.

    // write(true) is passed because new files must be created as write-accessible.
    // Otherwise a permissions error is thrown.

    // See https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create for more details
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            // If the file already exists, that's fine - we don't need to take an action
            AlreadyExists => Ok(()),
            // Otherwise, we propagate the error forward
            _ => Err(err),
        },
    }
}

pub fn create_path(path: &Path, kind: PathKind, options: UntreeOptions) -> PathResult<()> {
    let name = path.to_str().unwrap_or("<unprintable>");
    use PathContext::*;

    match (options.is_verbose(), kind) {
        (false, _) => {} // Print nothing if is_verbose() is false
        (_, FilePath) => println!("{} {}", "touch".bold().green(), name.bold().white()),
        (_, Directory) => println!("{} -p {}", "mkdir".bold().green(), name.bold().blue()),
    }

    match (options.dry_run, kind) {
        (true, _) => Ok(()), // Do nothing when dry_run is true
        (_, FilePath) => touch_file(path).add_context(CreateFile(path)),
        (_, Directory) => create_dir_all(path).add_context(CreateDirectory(path)),
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    let mut go_back = 0;
    for component in path.components() {
        match component.as_os_str().to_str() {
            Some(".") => {}
            Some("..") => {
                if !result.pop() {
                    go_back += 1;
                }
            }
            _ => result.push(component),
        }
    }
    if go_back > 0 {
        let mut prefix = PathBuf::new();
        for _ in 0..go_back {
            prefix.push("..");
        }
        prefix.push(result);
        result = prefix;
    }
    result
}

pub fn create_tree(
    directory: &String,
    mut lines: Lines<impl BufRead>,
    options: UntreeOptions
) -> PathResult<()> {
    let mut path: PathBuf = directory.into();

    let mut old_depth = match lines.next() {
        Some(Ok(line)) => {
            let (depth, filename) = get_entry(line.as_ref());
            path.push(filename);
            path = normalize_path(path.as_path());
            depth
        }
        Some(Err(err)) => return Err(err.into()),
        None => 0,
    };

    for result in lines {
        let line = match result { Err(err) => Err(err.into()), ok => ok }?;
        if line.is_empty() {
            break;
        }
        let (depth, filename) = get_entry(line.as_ref());
        if depth <= old_depth {
            create_path(path.as_path(), FilePath, options)?;
            for _ in depth..old_depth {
                path.pop();
            }
            path.set_file_name(filename);
        } else {
            create_path(path.as_path(), Directory, options)?;
            path.push(filename);
        }
        old_depth = depth;
    }
    create_path(path.as_path(), FilePath, options)
}
