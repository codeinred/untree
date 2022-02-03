use std::fmt::{self, Display};


#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum PathKind {
    FilePath,
    Directory,
}

impl Display for PathKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Display for UntreeOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
