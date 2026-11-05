use super::package::Package;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Output {
    pub packages: Vec<Package>,
}
