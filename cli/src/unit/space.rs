use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Space {
    #[default]
    User,
    Supervisor,
}
