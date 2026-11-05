use super::Space;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Loader {
    pub image: String,
    #[serde(default)]
    pub space: Space,
    #[serde(default)]
    pub args: Vec<u64>,
    #[serde(default)]
    pub stack: u64,
}
