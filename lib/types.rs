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

#[derive(Clone, Copy)]
pub enum PathKind {
    FilePath,
    Directory,
}
