use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::Path;

use quick_error::ResultExt;

use untree::*;

fn main() {
    use Error::*;
    if let Err(err) = run() {
        let header = "Error".bold().red();
        let prefix = "".bold().red();
        let line = format!("{:═<80}", "").bold().red();

        eprintln!();
        eprintln!("{header}");
        eprintln!("{line}");

        match err {
            MissingContext(err) => {
                eprintln!("Error with unknown context. {err}")
            }
            OnStdin(err) => {
                eprintln!("Error when attempting to read standard input. {err}")
            }
            OnPath(path, action, err) => {
                let path = path.to_str().unwrap_or("<unspeakable path>").bold();
                let action = match action {
                    CreateFile => format!("create file '{path}'"),
                    CreateDirectory => format!("create directory '{path}'"),
                    OpenFileForReading => format!("open '{path}' for reading"),
                    ReadFile => format!("read file '{path}'"),
                };
                eprintln!("{prefix}Error when attempting to {action}.");
                eprintln!("{prefix}");
                eprintln!("{prefix}{err}");
                eprintln!();
            }
        }

        std::process::exit(1);
    }
}

fn run() -> Result<()> {
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
        create_tree(&directory, read_stdin(), options).more_context(ReadStdin)
    } else {
        Ok(for file in tree_files {
            match file.as_str() {
                "-" => {
                    eprintln!(
                        "{}",
                        format!("Reading tree from standard input")
                            .red()
                            .bold()
                    );
                    create_tree(&directory, read_stdin(), options)
                        .more_context(ReadStdin)?;
                }
                file => {
                    let file = file.strip_prefix("\\").unwrap_or(file);
                    let path = Path::new(file);
                    let lines = read_lines(path)?;
                    eprintln!(
                        "{}",
                        format!("Reading tree from file '{file}'").red().bold()
                    );
                    create_tree(&directory, lines, options)
                        .more_context(ReadFile.on(path))?;
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
fn read_lines<'a>(path: &'a Path) -> Result<Lines<BufReader<File>>> {
    Ok(File::open(path)
        .map(|file| io::BufReader::new(file).lines())
        .context(OpenFileForReading.on(path))?)
}

/// A program to instantiate directory trees from the output of tree
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to use as the root of the newly generated directory
    /// structure. Uses current working directory if no directory is
    /// specified.
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
