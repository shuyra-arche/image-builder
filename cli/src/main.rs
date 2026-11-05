mod invocation;
mod operation;

use invocation::Invocation;
use operation::Operation;
use vanllia::{Error, reason::Reason};

fn run(invocation: Invocation) -> Result<(), Error> {
    let catalog = vanllia::load(&invocation.root)?;
    match invocation.operation {
        Operation::Check => println!(
            "checked: {} Units, {} Cargo bins",
            catalog.units().len(),
            catalog.images().len()
        ),
        Operation::Plan => {
            let deployment = vanllia::plan(&catalog, &invocation.selection)?;
            println!("boot: {}", deployment.system);
            for id in &deployment.order {
                let declaration = &catalog.units()[id];
                let image = &catalog.images()[&declaration.unit.loader.image];
                let reasons = deployment.reasons[id]
                    .iter()
                    .map(|reason| match reason {
                        Reason::Boot => "System boot entry".into(),
                        Reason::Selected => "selected".into(),
                        Reason::RequiredBy(owner) => format!("required by {owner}"),
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                println!("unit: {id} ({reasons})");
                println!("  declaration: {}", declaration.path.display());
                println!(
                    "  image: {} / package {} / {}",
                    image.name,
                    image.package,
                    image.manifest.display()
                );
                let after: Vec<_> = declaration
                    .unit
                    .control
                    .after
                    .iter()
                    .filter(|id| deployment.reasons.contains_key(*id))
                    .collect();
                println!("  after: {after:?}");
            }
            println!("one valid startup order: {}", deployment.order.join(" -> "));
        }
    }
    Ok(())
}

fn main() {
    let result = Invocation::parse(std::env::args().skip(1))
        .map_err(|error| error.to_string())
        .and_then(|invocation| {
            invocation
                .map(run)
                .transpose()
                .map_err(|error| error.to_string())
        });
    if let Err(error) = result {
        eprintln!("vanllia: {error}");
        std::process::exit(2);
    }
}
