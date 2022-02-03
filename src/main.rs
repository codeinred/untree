use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result, Stdin};
use std::path::{Path};

use untree::*;

fn main() -> PathResult<()> {
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
        Ok(for file in tree_files {
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
                    create_tree(&directory, read_lines("-").add_context("-")?, options)?;
                }
                file => {
                    eprintln!(
                        "{}",
                        format!("Reading tree from file '{file}'").red().bold()
                    );
                    create_tree(&directory, read_lines(file).add_context(file)?, options)?;
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
fn read_lines(file: impl AsRef<Path>) -> Result<Lines<BufReader<File>>> {
    File::open(file).map(|file| io::BufReader::new(file).lines())
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
