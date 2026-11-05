pub mod catalog;
pub mod config;
pub mod deployment;
pub mod discovery;
pub mod error;
pub mod image;
pub mod metadata;
pub mod planner;
pub mod reason;
pub mod selection;
pub mod source;
pub mod source_state;
pub mod unit;

pub use catalog::Catalog;
pub use deployment::Deployment;
pub use error::Error;
pub use planner::plan;
pub use selection::Selection;

/// Observe files and Cargo metadata; planning itself is pure.
pub fn load(root: &std::path::Path) -> Result<Catalog, Error> {
    let root = discovery::find_root(root)?;
    let project = config::read::<config::Project>(&root.join("vanllia.toml"))?;
    project.validate(&root.join("vanllia.toml"))?;
    let files = discovery::discover(&root, &project)?;
    let units = files
        .units
        .iter()
        .map(|path| {
            config::read::<unit::Unit>(path).map(|unit| unit::Declaration {
                path: path.clone(),
                unit,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let images = metadata::read(&files.manifests)?;
    catalog::index(project, units, images)
}
