use colored::*;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, ErrorKind::AlreadyExists, Lines};
use std::iter::Iterator;
use std::mem::replace;
use std::path::{Path, PathBuf};

use super::{PathKind::*, *};
use quick_error::ResultExt;

/// Returns an entry in the tree, where the first result is the depth, and the
/// second result is the file
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

/// Atomically create a file, if it doesn't already exist. This is an atomic
/// operation on the filesystem. If the file already exists, this function exits
/// without affecting that file.
pub fn touch_file(path: impl AsRef<Path>) -> Result<()> {
    // create_new is used to implement creation + existence checking as an
    // atomic filesystem operation.

    // create_new is used instead of create because the program should NOT
    // attempt to open files that already exist. This could result in an
    // exception being thrown if the file is locked by another program, or
    // marked as read only.

    // write(true) is passed because new files must be created as
    // write-accessible. Otherwise a permissions error is thrown.

    // See https://doc.rust-lang.org/std/fs/struct.OpenOptions.html#method.create for more details
    let path = path.as_ref();
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            // If the file already exists, that's fine - we don't need to take
            // an action
            AlreadyExists => Ok(()),
            // Otherwise, we propagate the error forward
            _ => Err(err).context(CreateFile.on(path))?,
        },
    }
}

/// Create a directory, along with any parents that haven't been created
pub fn touch_directory(path: impl AsRef<Path>) -> Result<()> {
    Ok(create_dir_all(path.as_ref())
        .context(CreateDirectory.on(path.as_ref()))?)
}

/// Create either a file (for `kind == PathKind::File`) or a directory (for
/// `kind == PathKind::Directory`). Provides additional options in the form of
/// `UntreeOptions`.
///
/// If `options.verbose` is set, print out the creation of the file or
/// directory. If `options.dry_run` is set, print out the creation of the file
/// or directory, but don't actually create it (`options.dry_run` implies
/// verbose)
pub fn create_path(
    path: impl AsRef<Path>,
    kind: PathKind,
    options: UntreeOptions,
) -> Result<()> {
    let path = path.as_ref();
    let name = path.to_str().unwrap_or("<unprintable>");

    match (options.is_verbose(), kind) {
        (false, _) => {} // Print nothing if is_verbose() is false
        (_, FilePath) => {
            println!("{} {}", "touch".bold().green(), name.bold().white())
        }
        (_, Directory) => {
            println!("{} -p {}", "mkdir".bold().green(), name.bold().blue())
        }
    }

    match (options.dry_run, kind) {
        (true, _) => Ok(()), // Do nothing when dry_run is true
        (_, FilePath) => touch_file(path),
        (_, Directory) => touch_directory(path),
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

/// Create a tree based on a sequence of lines describing the tree structure
/// inside the given directory
///
/// - `directory` - the directory in which to place the root of the tree
/// - `lines` - the source from which to read the tree
/// - `options` - options such as, whether to do a dry run, or whether to print
///    out what create_tree is doing
///
/// **Example:**
///
/// ```rust
/// use untree::*;
/// use std::io::{BufRead, BufReader, stdin, Lines};
///
/// let options = UntreeOptions::new()
///     .dry_run(true)
///     .verbose(true);
///
/// let lines = BufReader::new(stdin()).lines();
///
/// create_tree("path/to/directory", lines, options)?;
///
/// # Ok::<(), Error>(())
/// ```
pub fn create_tree(
    directory: impl AsRef<Path>,
    lines: Lines<impl BufRead>,
    options: UntreeOptions,
) -> Result<()> {
    iter_tree(directory, lines, |result| match result {
        Ok((path, kind)) => create_path(path, kind, options),
        Err(err) => Err(err),
    })
    .collect()
}

pub fn iter_tree<BR, F, T>(
    directory: impl AsRef<Path>,
    mut lines: Lines<BR>,
    func: F,
) -> impl Iterator<Item = T>
where
    BR: BufRead,
    F: FnMut(Result<(&Path, PathKind)>) -> T,
{
    enum State {
        Good,
        Empty,
        Bad(Error),
    }
    use State::*;
    struct It<BR, F, T>
    where
        F: FnMut(Result<(&Path, PathKind)>) -> T,
    {
        lines: Lines<BR>,
        state: State,
        path: PathBuf,
        depth: i32,
        old_depth: i32,
        filename: PathBuf,
        func: F,
    }
    impl<BR: BufRead, F, T> Iterator for It<BR, F, T>
    where
        F: FnMut(Result<(&Path, PathKind)>) -> T,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            match &self.state {
                Good => {}
                Empty => return None,
                Bad(_) => {
                    if let Bad(err) = replace(&mut self.state, Empty) {
                        return Some((self.func)(Err(err)));
                    } else {
                        unreachable!()
                    }
                }
            }
            if self.depth <= self.old_depth {
                for _ in self.depth..self.old_depth {
                    self.path.pop();
                }
                self.path.set_file_name(&self.filename);
            } else {
                self.path.push(&self.filename);
            }
            self.old_depth = self.depth;
            match self.lines.next() {
                Some(Ok(line)) => {
                    if line.is_empty() {
                        self.state = Empty;
                        let result = (self.path.as_path(), FilePath);
                        return Some((self.func)(Ok(result)));
                    }
                    let (new_depth, filename) = get_entry(line.as_ref());
                    let kind = if new_depth <= self.depth {
                        FilePath
                    } else {
                        Directory
                    };
                    self.depth = new_depth;
                    self.filename = filename.into();
                    let result = (self.path.as_path(), kind);
                    Some((self.func)(Ok(result)))
                }
                Some(Err(err)) => {
                    self.state = Bad(err.into());
                    let result = (self.path.as_path(), FilePath);
                    Some((self.func)(Ok(result)))
                }
                None => {
                    self.state = Empty;
                    let result = (self.path.as_path(), FilePath);
                    Some((self.func)(Ok(result)))
                }
            }
        }
    }
    match lines.next() {
        Some(Ok(line)) => {
            let (depth, filename) = get_entry(line.as_ref());
            let result: It<BR, F, T> = It {
                lines: lines,
                state: Good,
                path: directory.as_ref().into(),
                depth: depth,
                old_depth: -1,
                filename: normalize_path(filename.as_ref()),
                func,
            };
            return result;
        }
        Some(Err(err)) => {
            let result: It<BR, F, T> = It {
                lines: lines,
                state: Bad(err.into()),
                path: directory.as_ref().into(),
                depth: 0,
                old_depth: -1,
                filename: PathBuf::new(),
                func,
            };
            return result;
        }
        None => {
            let result: It<BR, F, T> = It {
                lines: lines,
                state: Empty,
                path: PathBuf::new(),
                depth: 0,
                old_depth: -1,
                filename: PathBuf::new(),
                func,
            };
            return result;
        }
    }
}
