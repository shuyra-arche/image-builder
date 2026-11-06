# Vanllia build tools

This repository provides the `vanllia` host CLI and the `vanllia_build` Rust
library. The CLI discovers provider-owned Unit TOML, resolves pinned sources,
builds Cargo binaries and packages initrd for System. It also supports local
source overlays and execution of exact saved artifacts. Product selection lives
in the consuming product's `vanllia.toml`, not in this repository.

## Source and interfaces

- `cli/src/`: the current `vanllia` executable; [CLI guide](cli/README.md).
- `src/lib.rs`, `src/capsule.rs`, `src/publication.rs`: static planning, ELF capsule
  encoding, runtime stripping, output leases and atomic publication.
- `src/workflow.rs`, `src/workflow/`: the retained JSON workflow library API.
- `tests/`, `cli/tests/`: library publication and CLI integration tests.
- `examples/`: fixtures for the retained library workflow, not CLI product defaults.

The library's static API remains available. The former `vanllia-build` executable
is absent; its old commands are not supported by the current CLI.

## Setup and verification

Use Linux, Git and Rustup/Cargo. `rust-toolchain.toml` pins
`nightly-2026-09-04`; the CLI manifest additionally declares its minimum Rust
version. Target builds need `riscv64gc-unknown-none-elf`, `rust-src` and
`llvm-objcopy` on PATH. Fixture tests can build real RV64 binaries. QEMU tests
need a separately identified firmware input.

```sh
cargo install --path cli --locked --target x86_64-unknown-linux-gnu
cargo test --workspace --locked --target x86_64-unknown-linux-gnu
cargo clippy --workspace --all-targets --locked --target x86_64-unknown-linux-gnu -- -D warnings
```

These are the repository's check entry points, not a claim of a fresh successful
run with the pinned toolchain. Validate a product separately after changing its
source pins. Host tests do not establish guest, hardware or SMP acceptance.

## Library guarantees and limits

[Workflow caller contract](docs/workflow-contract.md) covers source snapshots,
state isolation, runner ownership, failure reservations and publication receipts.

Published output uses a sibling temporary file and rename while an output lease
is held; failures preserve the previous successful artifact. This is local
filesystem atomicity, not a distributed or power-loss durability promise.
Library embedders own cancellation (`workflow::cancel`) and child reaping.
Dynamic coordinates used by the retained static API are interned for the process
lifetime; long-running embedders should bound configurations.

[Source attribution](PROVENANCE.md) and
[shared contribution rules](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
