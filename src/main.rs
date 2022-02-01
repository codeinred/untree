use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::{Path, PathBuf};

use colored::*;

mod macros;
use macros::either;

use clap::Parser;

#[derive(Clone, Copy)]
struct UntreeOptions {
    dry_run: bool,
    verbose: bool,
}

/// A program to create a directory structure from tree representations
/// of directories
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Directory in which to generate tree
    ///
    /// (Uses current working directory if no directory is specified)
    #[clap(short, long)]
    dir: Option<String>,
    /// Input file describing tree
    ///
    /// (read from stdin if no file is specified)
    #[clap()]
    tree_file: Option<String>,

    /// Print the names of files and directories without creating them.
    ///
    /// Implies verbose.
    #[clap(long)]
    dry_run: bool,

    /// Print out the names of files and directories that untree creates
    #[clap(long)]
    verbose: bool,
}

type IO = Result<(), io::Error>;
type IOResult<T> = Result<T, io::Error>;

fn main() -> IO {
    let args = Args::parse();

    let directory = match args.dir {
        None => String::from(""),
        Some(str) => str,
    };

    let options = UntreeOptions {
        dry_run: args.dry_run,
        verbose: args.verbose,
    };

    match args.tree_file {
        None => create_tree(directory, read_stdin(), options),
        Some(filename) => {
            if filename == "-" {
                create_tree(directory, read_stdin(), options)
            } else {
                create_tree(directory, read_lines(filename)?, options)
            }
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
fn get_entry(entry: &str) -> (i32, &str) {
    match either!(
        entry.strip_prefix("    "),
        entry.strip_prefix("└── "),
        entry.strip_prefix("├── "),
        entry.strip_prefix("│   "),
        // Some iplementations of tree use a non-breaking space here (ua0)
        entry.strip_prefix("│\u{a0}\u{a0} ")
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
    Ok(())
}

fn create_tree(directory: String, lines: Lines<impl BufRead>, options: UntreeOptions) -> IO {
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
    create_path(Path::new(&path), PathKind::File, options)?;

    Ok(())
}
