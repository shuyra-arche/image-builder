use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Target {
    pub name: String,
    pub kind: Vec<String>,
}
