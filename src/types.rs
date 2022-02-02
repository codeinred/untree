use clap::Parser;

#[derive(Clone, Copy)]
pub struct UntreeOptions {
    pub dry_run: bool,
    pub verbose: bool,
}

/// A program to instantiate directory trees from the output of tree
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Directory in which to generate tree
    ///
    /// (Uses current working directory if no directory is specified)
    #[clap(short, long)]
    pub dir: Option<String>,
    /// List of files containing trees to be read by untree
    ///
    /// (If no files are specified then the tree is read from stdin)
    #[clap()]
    pub tree_files: Vec<String>,

    /// Print the names of files and directories without creating them.
    ///
    /// Implies verbose.
    #[clap(long)]
    pub dry_run: bool,

    /// Print out the names of files and directories that untree creates
    #[clap(short, long)]
    pub verbose: bool,
}

pub enum PathKind {
    File,
    Directory,
}
