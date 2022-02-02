use clap::Parser;
use colored::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, ErrorKind::AlreadyExists, Lines, Stdin};
use std::path::{Path, PathBuf};

mod macros;
mod traits;
mod types;

use {macros::either, traits::Pure, types::*};

type IO = Result<(), io::Error>;
type IOResult<T> = Result<T, io::Error>;

fn main() -> IO {
    let args = Args::parse();

    let directory = args.dir.unwrap_or("".into());

    let options = UntreeOptions {
        dry_run: args.dry_run,
        verbose: args.verbose,
    };

    let tree_files = &args.tree_files;

    if tree_files.len() == 0 {
        eprintln!(
            "{}",
            format!("Reading tree from standard input").red().bold()
        );
        create_tree(&directory, read_stdin(), options)
    } else {
        for file in tree_files {
            match file.as_str() {
                "-" => {
                    eprintln!(
                        "{}",
                        format!("Reading tree from standard input").red().bold()
                    );
                    create_tree(&directory, read_stdin(), options)?;
                }
                "\\-" => {
                    eprintln!("{}", format!("Reading tree from file '-'").red().bold());
                    create_tree(&directory, read_lines("-")?, options)?;
                }
                file => {
                    eprintln!(
                        "{}",
                        format!("Reading tree from file '{file}'").red().bold()
                    );
                    create_tree(&directory, read_lines(file)?, options)?;
                }
            }
        }
        .pure()
    }
}

fn read_stdin() -> Lines<BufReader<Stdin>> {
    io::BufReader::new(io::stdin()).lines()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(file: impl AsRef<Path>) -> IOResult<Lines<BufReader<File>>> {
    File::open(file).map(|file| io::BufReader::new(file).lines())
}

/**
 * Returns an entry in the tree, where the first result is the depth, and the second result is the file
 */
fn get_entry(mut entry: &str) -> (i32, &str) {
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

// Atomically create a file, if it doesn't already exist. This is an atomic operation
fn atomic_create_file(path: &Path) -> IO {
    match OpenOptions::new()
        .read(true)
        .write(true)
        // Ensure that the file is only created if it doesn't already exist
        // This means that creation + existence checking is an atomic operation
        .create_new(true)
        .open(path)
    {
        Ok(_) => ().pure(),
        Err(err) => match err.kind() {
            // If the file already exists, that's fine - we don't need to take an action
            AlreadyExists => ().pure(),
            // Otherwise, we propagate the error forward
            _ => Err(err),
        },
    }
}

fn create_path(path: &Path, kind: PathKind, options: UntreeOptions) -> IO {
    let name = path.to_str().unwrap_or("<unprintable>");

    match (options.is_verbose(), kind) {
        (false, _) => {} // Print nothing if is_verbose() is false
        (_, PathKind::File) => println!("{} {}", "touch".bold().green(), name.bold().white()),
        (_, PathKind::Directory) => {
            println!("{} -p {}", "mkdir".bold().green(), name.bold().blue())
        }
    }

    match (options.dry_run, kind) {
        (true, _) => ().pure(), // Do nothing when dry_run is true
        (_, PathKind::File) => atomic_create_file(path),
        (_, PathKind::Directory) => std::fs::create_dir_all(path),
    }
}

fn create_tree(directory: &String, lines: Lines<impl BufRead>, options: UntreeOptions) -> IO {
    let mut path: PathBuf = directory.into();

    let mut old_depth = -1;
    for result in lines {
        let line = result?;
        if line == "" {
            break;
        }

        let (depth, filename) = get_entry(line.as_ref());
        if depth <= old_depth {
            create_path(path.as_path(), PathKind::File, options)?;
            for _ in depth..old_depth {
                path.pop();
            }
            path.set_file_name(filename);
        } else {
            create_path(path.as_path(), PathKind::Directory, options)?;
            path.push(filename);
        }
        old_depth = depth;
    }
    create_path(path.as_path(), PathKind::File, options)
}
