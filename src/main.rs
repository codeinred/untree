#![cfg(feature = "build-binary")]

use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::{Path, PathBuf};
use textwrap::*;

use quick_error::ResultExt;

use clap::{AppSettings, FromArgMatches, IntoApp, Parser};

use untree::*;

fn main() {
    let app = Args::into_app()
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::NextLineHelp)
        .setting(AppSettings::DisableHelpSubcommand)
    //     .bold(Style::Good, true)
    //     .bold(Style::Warning, true)
    //     .foreground(Style::Warning, Some(Color::Green))
        .max_term_width(100);

    let args = Args::from_arg_matches(&app.get_matches()).unwrap();

    use Error::*;
    if let Err(err) = run(args) {
        match err {
            MissingContext(err) => {
                print_error("An error occured with unknown context.", err);
            }
            OnStdin(err) => {
                print_error("An error occured while attempting to read from standard input.", err);
            }
            OnPath(path, action, err) => {
                let path = path.to_str().unwrap_or("<unspeakable>").bold();
                let action = action.describe(&path);
                print_error(
                    format!("An error occured while attempting to {action}."),
                    err,
                );
            }
        }

        std::process::exit(1);
    }
}

fn print_error(msg: impl std::fmt::Display, base_err: io::Error) {
    let msg = format!("{msg}\n\nCause: {base_err}");
    let width = termwidth();

    let msg = fill(msg.as_str(), width - 4);
    let msg = indent(msg.as_str(), "    ");

    let header = "ERROR:".bold().red();

    eprintln!();
    eprintln!("{header}");
    eprintln!("{}", msg);
    eprintln!();
}

fn run(args: Args) -> Result<()> {
    let directory = args.dir.unwrap_or_else(|| "".into());

    let options = UntreeOptions::new()
        .dry_run(args.dry_run)
        .verbose(args.verbose);

    let tree_files = &args.tree_files;

    if tree_files.is_empty() {
        eprintln!("{}", "Reading tree from standard input".bold());
        create_tree(&directory, read_stdin(), options).more_context(ReadStdin)
    } else {
        for path in tree_files {
            let filename = path.to_str().unwrap_or("<unspeakable>");
            if filename == "-" {
                eprintln!("{}", "Reading tree from standard input".bold());
                create_tree(&directory, read_stdin(), options)
                    .more_context(ReadStdin)?;
            } else {
                let path = path.strip_prefix("\\").unwrap_or(path);
                let lines = read_lines(path)?;
                eprintln!(
                    "{}",
                    format!("Reading tree from file '{filename}'").bold()
                );
                create_tree(&directory, lines, options)
                    .more_context(ReadFile.on(path))?;
            }
        }
        Ok(())
    }
}

fn read_stdin() -> Lines<BufReader<Stdin>> {
    io::BufReader::new(io::stdin()).lines()
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(path: &'_ Path) -> Result<Lines<BufReader<File>>> {
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
    #[clap(short, long("--dir"), parse(from_os_str))]
    pub dir: Option<PathBuf>,
    /// List of files containing trees to be read by untree. If no files are
    /// specified, then the tree is read from standard input.
    #[clap(parse(from_os_str))]
    pub tree_files: Vec<PathBuf>,

    /// Print the names of files and directories without creating them.
    /// Implies verbose.
    #[clap(long)]
    pub dry_run: bool,

    /// Print out the names of files and directories that untree creates.
    #[clap(short, long)]
    pub verbose: bool,
}
