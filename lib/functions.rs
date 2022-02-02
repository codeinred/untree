use colored::*;
use std::fs::OpenOptions;
use std::io::{BufRead, ErrorKind::AlreadyExists, Lines, Result};
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

/// Atomically create a file, if it doesn't already exist. This is an atomic operation
pub fn atomic_create_file(path: &Path) -> Result<()> {
    // Ensure that the file is only created if it doesn't already exist
    // This means that creation + existence checking is an atomic operation
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

pub fn create_path(path: &Path, kind: PathKind, options: UntreeOptions) -> Result<()> {
    let name = path.to_str().unwrap_or("<unprintable>");

    match (options.is_verbose(), kind) {
        (false, _) => {} // Print nothing if is_verbose() is false
        (_, FilePath) => println!("{} {}", "touch".bold().green(), name.bold().white()),
        (_, Directory) => println!("{} -p {}", "mkdir".bold().green(), name.bold().blue()),
    }

    match (options.dry_run, kind) {
        (true, _) => Ok(()), // Do nothing when dry_run is true
        (_, FilePath) => atomic_create_file(path),
        (_, Directory) => std::fs::create_dir_all(path),
    }
}

pub fn create_tree(
    directory: &String,
    lines: Lines<impl BufRead>,
    options: UntreeOptions,
) -> Result<()> {
    let mut path: PathBuf = directory.into();

    let mut old_depth = -1;
    for result in lines {
        let line = result?;
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
