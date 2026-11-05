use super::{Boot, Source};
use crate::Error;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub schema: u32,
    #[serde(default)]
    pub units: Vec<String>,
    pub boot: Boot,
    #[serde(default)]
    pub sources: BTreeMap<String, Source>,
}

impl Project {
    pub fn validate(&self, path: &Path) -> Result<(), Error> {
        if self.schema != 1 || self.boot.system.is_empty() {
            return Err(Error::Declaration {
                path: path.to_path_buf(),
                message: "expected schema = 1 and a nonempty boot.system".into(),
            });
        }
        if self.sources.keys().any(|key| key.is_empty()) || self.units.iter().any(String::is_empty)
        {
            return Err(Error::Declaration {
                path: path.to_path_buf(),
                message: "source names and Unit IDs must be nonempty".into(),
            });
        }
        Ok(())
    }
}
