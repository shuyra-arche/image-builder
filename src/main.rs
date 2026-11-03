use std::path::Path;

fn main() {
    // This marker is only meaningful in children launched by this invocation.
    unsafe { std::env::remove_var("VANLLIA_MANAGED_GROUP") };
    // Reap orphaned grandchildren from an owned Cargo/runner process group.
    if unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1) } != 0 {
        eprintln!(
            "cannot establish child reaping: {}",
            std::io::Error::last_os_error()
        );
        std::process::exit(2);
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 4 || args[0] != "--config" || args[2] != "--state" {
        eprintln!(
            "usage: vanllia-build --config FILE --state DIR prepare|check|build DEPLOYMENT|run DEPLOYMENT|test NAME|cluster NAME|override set SOURCE CHECKOUT|override clear SOURCE|override check|inspect"
        );
        std::process::exit(2);
    }
    if let Err(error) = ctrlc::set_handler(vanllia_build::workflow::cancel) {
        eprintln!("signal handler: {error}");
        std::process::exit(2);
    }
    match vanllia_build::workflow::execute(Path::new(&args[1]), Path::new(&args[3]), &args[4..]) {
        Ok(path) => println!("ok  {}", path.display()),
        Err(error) => {
            eprintln!("vanllia-build: {error}");
            std::process::exit(2);
        }
    }
}
