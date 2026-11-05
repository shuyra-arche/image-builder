use crate::operation::Operation;
use std::path::PathBuf;
use vanllia::Selection;

pub(crate) struct Invocation {
    pub operation: Operation,
    pub root: PathBuf,
    pub selection: Selection,
}

impl Invocation {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>, String> {
        let mut args = args.into_iter();
        let mut operation = None;
        let mut root = None;
        let mut units = Vec::new();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    println!(
                        "usage: vanllia check|plan [--root DIRECTORY] [--unit ID ...]\ncheck/plan inspect prepared local sources without compiling or starting a guest"
                    );
                    return Ok(None);
                }
                "--version" | "-V" => {
                    println!("vanllia {}", env!("CARGO_PKG_VERSION"));
                    return Ok(None);
                }
                "--root" => {
                    root = Some(PathBuf::from(
                        args.next().ok_or("--root needs a directory")?,
                    ))
                }
                "--unit" => units.push(
                    args.next()
                        .filter(|id| !id.is_empty())
                        .ok_or("--unit needs a Unit ID")?,
                ),
                "check" if operation.is_none() => operation = Some(Operation::Check),
                "plan" if operation.is_none() => operation = Some(Operation::Plan),
                _ => return Err(format!("unknown argument `{arg}`; use vanllia --help")),
            }
        }
        let operation = operation.ok_or("expected check or plan; use vanllia --help")?;
        if matches!(operation, Operation::Check) && !units.is_empty() {
            return Err(
                "check validates all discovered declarations; use plan --unit to select Units"
                    .into(),
            );
        }
        Ok(Some(Self {
            operation,
            root: root.unwrap_or(std::env::current_dir().map_err(|error| error.to_string())?),
            selection: if units.is_empty() {
                Selection::Default
            } else {
                Selection::Explicit(units)
            },
        }))
    }
}
