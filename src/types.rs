use clap::Parser;

#[derive(Clone, Copy)]
pub struct UntreeOptions {
    pub dry_run: bool,
    pub verbose: bool,
}

/// A program to create a directory structure from tree representations
/// of directories
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Directory in which to generate tree
    ///
    /// (Uses current working directory if no directory is specified)
    #[clap(short, long)]
    pub dir: Option<String>,
    /// Input file describing tree
    ///
    /// (read from stdin if no file is specified)
    #[clap()]
    pub tree_file: Option<String>,

    /// Print the names of files and directories without creating them.
    ///
    /// Implies verbose.
    #[clap(long)]
    pub dry_run: bool,

    /// Print out the names of files and directories that untree creates
    #[clap(long)]
    pub verbose: bool,
}

pub enum PathKind {
    File,
    Directory,
}
