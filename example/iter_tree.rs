use clap::Parser;
use colored::*;
use quick_error::ResultExt;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
};
use untree::*;

/// A program to instantiate directory trees from the output of tree
#[derive(Parser, Debug)]
#[clap(
    author,
    name("iter_tree"),
    version,
    about("An example program to illustrate iter_tree()"),
    long_about = None)]
pub struct Args {
    /// Directory to use as the root of the newly generated directory
    /// structure. Uses current working directory if no directory is
    /// specified.
    #[clap(short, long("--dir"), parse(from_os_str))]
    pub dir: Option<PathBuf>,
    /// File to read the tree from
    #[clap(parse(from_os_str))]
    pub tree_file: PathBuf,
}

/// The output is wrapped in a Result to allow matching on errors
/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(path: &'_ Path) -> Result<Lines<BufReader<File>>> {
    Ok(File::open(path)
        .map(|file| io::BufReader::new(file).lines())
        .context(OpenFileForReading.on(path))?)
}

fn main() -> untree::Result<()> {
    let args = Args::parse();

    let dir = args.dir.unwrap_or(PathBuf::new());
    let lines = read_lines(args.tree_file.as_path())?;

    let iter = iter_tree(dir.clone(), lines, |path| match path {
        Ok((path, kind)) => {
            let path = path.to_str().unwrap_or("<unspeakable>").green().bold();
            let kind = match kind {
                PathKind::FilePath => kind.to_string().bold(),
                PathKind::Directory => kind.to_string().blue().bold(),
            };
            println!("  {:<10} {:<20}", kind, path);
            Ok(())
        }
        Err(err) => Err(err),
    });

    let result: Result<()> = iter.collect();
    result.more_context(untree::ReadFile.on(dir.as_path()))
}
