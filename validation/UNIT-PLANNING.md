# Unit discovery and planning checkpoint

Base: `17fd7ddbd18b3aef3a8d348cd32d8642e81b3c7f` on the repository's default
branch, `codex/source-snapshot`.

The implementation adds a standalone `vanllia` package under `cli/`, without a
dependency on the legacy static `UnitFile`, `Deployment`, provider registry, or
image coordinate tables. Catalog indexing and selection planning are pure
functions; source availability is explicitly modeled as available or missing.
Deployment is an owned invocation value and has no file serialization path.

## Verification

Host target: `x86_64-unknown-linux-gnu`.

The selected environment provides `nightly-2026-10-09`; validation explicitly
used that installed toolchain via the environment. The historical repository
pin remains `nightly-2026-09-04` and was not changed. This checkpoint does not
claim a run under that older pin.

- `cargo test -p vanllia --target x86_64-unknown-linux-gnu --locked --offline`:
  18 tests passed (10 planning tests, 8 discovery/invocation tests).
- `cargo test --workspace --lib --target x86_64-unknown-linux-gnu --locked`:
  20 existing host library tests passed.
- `cargo clippy -p vanllia --all-targets --target x86_64-unknown-linux-gnu --locked --offline -- -D warnings`:
  passed.
- `cargo fmt -p vanllia -- --check`: passed.
- `git diff --check`: passed.
- `cargo install --path cli --root /workspace/.vanllia-tools --locked --target x86_64-unknown-linux-gnu`:
  installed the independent host executable.

Discovery tests use real Cargo metadata and a fixture build script that would
panic if executed. They verify no fixture build outputs or Cargo lock file were
created. Other cases cover custom Git-ignored outputs, symlinks, nested repository
ownership, missing submodules, separately declared local sources, and invocation
from a component directory. Pure planning tests cover mutual `requires`, inactive
`after` edges, selected and unselected cycles, explicit selection replacing
defaults, unknown references, identity conflicts and the independent System
boot entry.

No QEMU invocation or guest/device acceptance was performed. The existing
real-RISC-V artifact integration suite was not run for this host planning slice.

## Remaining implementation

This is the first code checkpoint, not the completed build-system migration.
Remote source preparation and locking, local Cargo dependency patching, full
runtime Unit semantics, binary encoding, dynamic System consumption, product
build/run/test commands and replacement of legacy entry points remain. Their
absence is documented in `cli/README.md`; unsupported declaration fields are
errors, not ignored configuration.
