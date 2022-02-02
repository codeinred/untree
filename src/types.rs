use clap::Parser;

#[derive(Clone, Copy)]
pub struct UntreeOptions {
    pub dry_run: bool,
    pub verbose: bool,
}

impl UntreeOptions {
    // Check if either self.verbose or self.dry_run is true.
    // If dry_run is true, then verbose should be implied as true
    pub fn is_verbose(&self) -> bool {
        return self.verbose || self.dry_run;
    }
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

#[derive(Clone, Copy)]
pub enum PathKind {
    FilePath,
    Directory,
}
