# image-builder

Host-only image-builder core, published as package `image-builder` / library
`image_builder`. It turns explicit unit declarations plus explicit Cargo image
coordinates into one initrd: a manifest of loadable ELF images followed by the
paged bootstrap capsule. `ImageBuild.image`, `package`, `bin`, `features`,
`target`, `profile` and `default_features` remain separate coordinates; Cargo's
reported `package_id`, bin target, non-test profile and executable remain the
authority for every artifact.

## Public surface

- `executables(root, work, builds)` — resolve real Cargo executables.
- `build_with(root, work, output, declarations, deployment, builds)` — build,
  slim, pack the manifest and append the capsule.
- `image_inventory(declarations, deployment, builds)` — the images a deployment
  needs, with their `ProgramKind`.
- `startup_order(declarations, deployment)` — the validated startup sequence.
- `pub use unit::ImageBuild` — the image coordinate.

The caller supplies the build root, nested target directory and output path; no
repository or composition layout is inferred. Nothing here references the
Source repository, its composition catalog, or a System runtime SDK.

Planning computes the declaration closure and startup graph; it does not resolve
Cargo artifacts or replace the coordinate-uniqueness validation performed by
`build_with`/`executables`. Multiple logical images may resolve to the same path.

This is a public Git-revision release, not a crates.io release. The repository
keeps the Source toolchain checkpoint `nightly-2026-09-04` for reproducible real
RISC-V fixtures; no stable-Rust/MSRV guarantee is asserted. After a normal
`cargo fetch --locked` for this repository and its nested fixture, run host tests
with explicit `--target x86_64-unknown-linux-gnu --locked --offline` and put the
toolchain's llvm-objcopy on PATH.

## Test ownership

- Provider (this repo): 9 pure inline cases (planning, validation, ELF check) and
  5 real-Cargo-artifact cases over `tests/fixtures/link-workspace` (multi-package,
  feature variants, release profile, alias fan-out). The fixture depends only on
  the public `env`/`execution`/`mold` revisions; its `release`/`release-check`
  profiles keep `debug-assertions = true`.
- Source Integration: 9 actual-composition cases and 3 catalog cases live in the
  Source tree, not here.

## Behavior notes

- Cargo runs with `--locked`; each coordinate group gets its own
  `work/group-N` target directory, which is rebuildable Cargo cache.
- Distinct image names that share one Cargo coordinate receive the same resolved
  executable; paths are never guessed and valid aliases are not rejected.
- Progress lines go to stdout (`initrd: …`, `image resolved: …`); the Source CLI
  prints its final `ok  <path>` last.
- The final `std::fs::write` is not atomic: a failed write can leave a partial
  output at the requested path. A returned `Err` means that file is not valid.
  The caller serializes use of one work/output pair.

## Provenance

Mechanically extracted from the Source tree `anreonyr/sqware` at full commit
`aaed018fb2c6b9088f2d06fb6a8d5e23f984a21a` (`crates/image`). Repository,
publication and provenance records are owned by the extraction root; this
repository does not invent a license.
