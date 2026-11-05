mod boot;
mod project;
mod source;

pub use boot::Boot;
pub use project::Project;
pub use source::Source;

use crate::{Error, error};
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
    let text = std::fs::read_to_string(path).map_err(|e| error::io(path, e))?;
    toml_edit::de::from_str(&text).map_err(|e| Error::Declaration {
        path: path.to_path_buf(),
        message: e.to_string(),
    })
}
