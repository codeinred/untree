use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::Path;

use untree::{PathContext::*, *};

fn main() {
    match run() {
        Err(err) => match err.context {
            Missing => eprintln!("\nError with unknown source. {}", err.base_error),
            Stdin => eprintln!("\nError reading from standard input. {}", err.base_error),
            Read(path) => eprintln!(
                "\nError reading file '{}'. {}",
                path.to_str().unwrap_or("<unprintable>").bold(),
                err.base_error
            ),
            CreateFile(path) => eprintln!(
                "\nError creating file '{}'. {}",
                path.to_str().unwrap_or("<unprintable>").bold(),
                err.base_error
            ),
            CreateDirectory(path) => eprintln!(
                "\nError creating directory '{}'. {}",
                path.to_str().unwrap_or("<unprintable>").blue().bold(),
                err.base_error
            ),
        },
        _ => {}
    }
}
fn run() -> PathResult<()> {
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
        create_tree(&directory, read_stdin(), options).supply_missing(Stdin)
    } else {
        Ok(for file in tree_files {
            match file.as_str() {
                "-" => {
                    eprintln!(
                        "{}",
                        format!("Reading tree from standard input").red().bold()
                    );
                    create_tree(&directory, read_stdin(), options).supply_missing(Stdin)?;
                }
                file => {
                    let file = file.strip_prefix("\\").unwrap_or(file);
                    eprintln!(
                        "{}",
                        format!("Reading tree from file '{file}'").red().bold()
                    );
                    create_tree(&directory, read_lines(file)?, options)
                        .supply_missing(Read(file.into()))?;
                }
            }
        })
    }
}

fn read_stdin() -> Lines<BufReader<Stdin>> {
    io::BufReader::new(io::stdin()).lines()
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<F>(file: F) -> untree::PathResult<Lines<BufReader<File>>>
where
    F: AsRef<Path> + Into<std::path::PathBuf>,
{
    File::open(file.as_ref())
        .map(|file| io::BufReader::new(file).lines())
        .add_context(Read(file))
}

/// A program to instantiate directory trees from the output of tree
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to use as the root of the newly generated directory structure.
    /// Uses current working directory if no directory is specified.
    #[clap(short, long)]
    pub dir: Option<String>,
    /// List of files containing trees to be read by untree. If no files are
    /// specified, then the tree is read from standard input.
    pub tree_files: Vec<String>,

    /// Print the names of files and directories without creating them.
    /// Implies verbose.
    #[clap(long)]
    pub dry_run: bool,

    /// Print out the names of files and directories that untree creates.
    #[clap(short, long)]
    pub verbose: bool,
}
