use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub after: Vec<String>,
}
