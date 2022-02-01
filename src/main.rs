use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
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

    match args.tree_file.as_ref().map(String::as_str) {
        None | Some("-") => {
            eprintln!(
                "{}",
                format!("Reading tree from standard input").red().bold()
            );
            create_tree(directory, read_stdin(), options)
        }
        Some(filename) => {
            eprintln!("{}", format!("Reading tree from {filename}").red().bold());
            create_tree(directory, read_lines(filename)?, options)
        }
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

fn create_path(path: &Path, kind: PathKind, options: UntreeOptions) -> IO {
    let name = path.to_str().unwrap_or("<unprintable>");
    if options.dry_run || options.verbose {
        match kind {
            PathKind::File => println!("{} {}", "touch".bold().green(), name.bold().white()),
            PathKind::Directory => println!("{} -p {}", "mkdir".bold().green(), name.bold().blue()),
        }
    }
    if !options.dry_run {
        match kind {
            PathKind::File => {
                if !path.exists() {
                    File::create(&path)?;
                }
            }
            PathKind::Directory => std::fs::create_dir_all(path)?,
        }
    }
    .pure()
}

fn create_tree(directory: String, lines: Lines<impl BufRead>, options: UntreeOptions) -> IO {
    let mut path: PathBuf = directory.into();

    let mut old_depth = -1;
    for result in lines {
        let line = result?;
        if line == "" {
            break;
        }

        let (depth, filename) = get_entry(line.as_ref());
        if depth <= old_depth {
            create_path(Path::new(&path), PathKind::File, options)?;
            for _ in depth..old_depth {
                path.pop();
            }
            path.set_file_name(filename);
        } else {
            create_path(Path::new(&path), PathKind::Directory, options)?;
            path.push(filename);
        }
        old_depth = depth;
    }
    create_path(Path::new(&path), PathKind::File, options)
}
