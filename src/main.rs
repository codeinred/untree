use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::{Path, PathBuf};
use std::vec::Vec;

use colored::*;

mod macros;
use macros::either;

type IO = Result<(), io::Error>;
type IOResult<T> = Result<T, io::Error>;

fn main() -> IO {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => create_tree(".", read_stdin(), true),
        2 => match args[1].as_ref() {
            "-" => create_tree(".", read_stdin(), true),
            "-h" | "--help" => Ok(print_help(args)),
            filename => create_tree(".", read_lines(filename)?, true),
        },
        _ => Ok(print_help(args)),
    }
}

fn print_help(args: Vec<String>) {
    let program_name = if args.len() == 0 {
        "<unknown>"
    } else {
        &args[0]
    };
    println!();
    println!("Usage:");
    println!();
    println!("    {program_name} [<filename>]");
    println!();
    println!("Reads lines from standard input, unless a filename is provided.");
    println!("If the filename is '-', standard input is used as the file.");
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
fn get_entry(entry: &str) -> (i32, &str) {
    match either!(
        entry.strip_prefix("    "),
        entry.strip_prefix("└── "),
        entry.strip_prefix("├── "),
        entry.strip_prefix("│   ")
    ) {
        Some(suffix) => {
            let (i, result) = get_entry(suffix);
            (i + 1, result)
        }
        None => (0, entry),
    }
}
enum PathKind {
    File,
    Directory,
}
fn create_path(path: &Path, kind: PathKind, dry_run: bool) -> IO {
    let name = path.to_str().unwrap_or("<unprintable>");
    match kind {
        PathKind::File => println!("{} {}", "touch".bold().green(), name.bold().white()),
        PathKind::Directory => println!("{} -p {}", "mkdir".bold().green(), name.bold().blue()),
    }
    if !dry_run {
        match kind {
            PathKind::File => {
                if !path.exists() {
                    OpenOptions::new().create_new(true).open(&path)?;
                }
            }
            PathKind::Directory => std::fs::create_dir_all(path)?,
        }
    }
    Ok(())
}

fn create_tree(directory: &str, lines: Lines<impl BufRead>, dry_run: bool) -> IO {
    let mut path = PathBuf::from(directory);

    let mut old_depth = -1;
    for result in lines {
        let line = result?;
        if line == "" {
            // We're done
            return Ok(());
        }

        let (depth, filename) = get_entry(line.as_ref());
        if depth <= old_depth {
            create_path(Path::new(&path), PathKind::File, dry_run)?;
            for _ in depth..old_depth {
                path.pop();
            }
            path.set_file_name(filename);
        } else {
            create_path(Path::new(&path), PathKind::Directory, dry_run)?;
            path.push(filename);
        }
        old_depth = depth;
    }
    create_path(Path::new(&path), PathKind::File, dry_run)?;

    Ok(())
}
