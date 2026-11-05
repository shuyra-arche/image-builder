use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct SourceFiles {
    pub units: BTreeSet<PathBuf>,
    pub manifests: BTreeSet<PathBuf>,
}
