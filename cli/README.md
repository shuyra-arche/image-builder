# Vanllia CLI: declaration discovery and planning

This is the first implementation slice of the new standalone `vanllia` CLI.
It supports `check` and `plan` over prepared source checkouts. Building product
ELFs, preparing remote sources, encoding runtime Units, and booting System are
subsequent slices; these commands do not claim to validate those behaviors.

Install the host executable from this checkout:

```sh
cargo install --path cli --locked
vanllia check --root /path/to/product
vanllia plan --root /path/to/product
vanllia plan --root /path/to/product --unit test.pipe-roundtrip
```

The CLI searches upwards from the current directory or `--root` for
`vanllia.toml`. Repeated `--unit ID` arguments replace the product's default root
selection. System is always included as the separately declared boot entry.
An absent default selection requires an explicit selection, which can select
System alone. `check` validates the entire discovered catalog; `plan` validates
the selected startup graph and prints the selection reasons and Cargo bindings.

## Product input

The current draft schema covers the inputs needed for Unit discovery and
planning. Kernel, firmware and platform configuration will be added together
with the executable build path, rather than accepted without validation.

```toml
schema = 1
units = ["pipe"]

[boot]
system = "system"

[sources.pipe]
path = "components/pipe"
```

The product root is itself a source. Explicit source paths are relative to the
entry configuration directory and may refer to separately checked-out local
repositories. Initialized Git submodules are discovered recursively. Missing
declared sources or uninitialized submodules are errors. This slice does not
fetch them or advance their versions. Source configuration names identify
checkouts, not a list of their Units.

Only the entry `vanllia.toml` supplies product defaults. A dependency checkout's
own product configuration does not add its default Units to this invocation.

## Unit input

This draft covers the runtime identity, loader configuration, and selection and
ordering dependencies. Operator, identity, supply and the remaining control
semantics will be introduced with the runtime codec; they are currently rejected
as unsupported fields rather than silently discarded.

```toml
schema = 1
id = "test.pipe-roundtrip"

[loader]
image = "test-pipe-roundtrip"
space = "user"
args = []
stack = 0

[control]
requires = ["pipe"]
after = ["pipe"]
```

`loader.image` names a Cargo bin target. Manifest paths, Cargo package identity
and source ownership are obtained from Cargo metadata; Unit declarations do not
repeat them. Several Units can reference one bin. Distinct bins with the same
name are ambiguous and rejected. Unit IDs are separately unique across the
selected source scope, independently of file location.

`requires` only grows the necessary set. Mutual `requires` edges are permitted
and terminate through set closure. `after` orders selected Units and never adds
Units to that set. An `after` target present in the catalog but absent from the
selection contributes no edge. Every reference must nevertheless resolve to a
known Unit. Only cycles in the selected `after` graph block the plan. The printed
topological order is one deterministic valid order, not an additional waiting
dependency between adjacent entries. The System boot entry cannot require or
wait for Units that it has not loaded yet.

## Discovery and execution boundaries

- Git source roots use tracked and untracked, nonignored files. Ordinary
  directories are recursively scanned. `.git`, `target`, `.vanllia`, `.cache`,
  `node_modules`, and `vendor` trees are excluded.
- Directory and file symlinks are not followed during discovery. Unregistered
  nested Git repositories are skipped; a separately declared local source or
  initialized submodule establishes its own boundary.
- `cargo metadata --no-deps --locked --offline --format-version 1` obtains bin
  targets. Only manifests actually discovered in the declared source scope are
  indexed, even when a Cargo workspace contains additional members outside it.
- Discovery observes filesystem and Cargo facts. Catalog validation and the
  planner are pure functions over owned declarations and these facts.
- Source availability is represented by explicit enum variants. The indexed
  catalog is a validated input phase; the resulting `Deployment` is an owned,
  in-memory value. It has no serialization implementation and is not written to
  a deployment file.
- No product compilation, build script, QEMU process, product output, or lockfile
  update is performed by `check` or `plan`.

Run the host tests using the configured toolchain:

```sh
cargo test -p vanllia --target x86_64-unknown-linux-gnu --locked
cargo clippy -p vanllia --all-targets --target x86_64-unknown-linux-gnu --locked -- -D warnings
cargo fmt -p vanllia -- --check
```

The Rust tests exercise selection closure, startup constraints, declaration
conflicts, repository boundaries, real Cargo metadata, and invocation from a
component directory. A fixture build script deliberately fails if executed,
ensuring planning does not accidentally compile fixture products.
