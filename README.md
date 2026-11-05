# Vanllia build system

Install the independent [`vanllia` CLI](cli/README.md) from `cli/`. It discovers
Unit declarations and real Cargo bins, prepares Git sources, constructs an
in-memory Deployment, builds the required images and packages binary Unit records
in initrd for System. It also provides local workspace overlays, Rust integration
cases and execution of exact saved artifacts.

The `vanllia_build` host library retains its earlier static API for existing
library consumers and provides ELF capsule encoding, stripping, atomic
publication and owned process cleanup. The old `vanllia-build` executable and
its CLI-specific tests are retired. Existing library tests remain.

See [validation/DYNAMIC-BUILD.md](validation/DYNAMIC-BUILD.md) for actual checks,
toolchain details and guest acceptance limits. Historical workflow documentation
and examples describe the earlier library interface; the CLI README is the
current product interface.
