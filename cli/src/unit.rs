mod control;
mod declaration;
mod loader;
mod space;

pub use control::Control;
pub use declaration::Declaration;
pub use loader::Loader;
pub use space::Space;

use crate::Error;
use serde::Deserialize;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unit {
    pub schema: u32,
    pub id: String,
    pub loader: Loader,
    #[serde(default)]
    pub control: Control,
}

impl Unit {
    pub fn validate(&self, path: &Path) -> Result<(), Error> {
        let fail = |message: &str| Error::Declaration {
            path: path.to_path_buf(),
            message: message.into(),
        };
        if self.schema != 1 {
            return Err(fail("expected Unit schema = 1"));
        }
        if self.id.is_empty() || self.loader.image.is_empty() {
            return Err(fail("Unit id and loader.image must be nonempty"));
        }
        for (label, names) in [
            ("requires", &self.control.requires),
            ("after", &self.control.after),
        ] {
            let unique: std::collections::BTreeSet<_> = names.iter().collect();
            if names.iter().any(String::is_empty) || unique.len() != names.len() {
                return Err(fail(&format!(
                    "{label} must contain distinct nonempty Unit IDs"
                )));
            }
        }
        Ok(())
    }
}
