# Managed Vanllia development

One Cargo package provides the library and CLI. Cargo owns package resolution,
locks, fetching, compilation, scheduling and caches; there is no second
Integration tool crate, task language, compilation cache or remote execution.

## Short workflow

Install `nightly-2026-09-04`, its host/RISC-V targets, llvm-objcopy and the
caller's declared runner. Host commands explicitly select the host target.

```sh
export RUSTUP_TOOLCHAIN=nightly-2026-09-04
cargo test --locked --target x86_64-unknown-linux-gnu
cargo run --locked --target x86_64-unknown-linux-gnu -- \
  --config examples/pipe/vanllia.json --state /tmp/vanllia prepare
```

With the same prefix through `--state /tmp/vanllia`, use:

```text
check                                  validate actual declarations
test pipe                              run the Unit's original host tests
override set pipe /path/to/pipe         request a local source snapshot
override check                         verify every workspace's Cargo resolution
run pipe-independent                   build/package/run the declared deployment
cluster pipe-unit                      run declared collaboration scenarios
override clear pipe                    select the fixed baseline again
build pipe-independent                 publish artifacts without runtime acceptance
inspect                                show context and last published deployment
```

`override set` records intent; `override check` verifies actual Cargo metadata.
The user checkout is read only: no fetch, checkout, reset, lock edit or cleanup.
Tracked and nonignored untracked files, deletions and executable modes are
snapshotted. Gitlinks are separate declared sources; symlinks/special files fail
preparation. Source/library snapshots are checked for changes during preparation.
Changed local bytes select a distinct context. Clearing substitution selects the
fixed baseline without restoring files over a shared lock.

## Explicit product configuration

`examples/pipe/vanllia.json` is caller-supplied data, not a catalog in the tool.
It pins pipe, independent Kernel and hashed firmware. Main workspace, `tests`
and `integration` remain distinct. UNIT/BUILD and Deployment stay in the existing
declaration package/host. `host.patch` migrates that consumer to `vanllia_build`
and `collect`; its private manifest points to a frozen copy of this package.
The declarations host, independent server and consumer images are rebuilt.
No host links the server implementation; pipe's own tests retain their existing
projection of their own ledger sources.

Configuration is strict JSON (unknown fields fail). Each source needs a
credential-free HTTPS repository/full commit. Workspaces list participating Cargo
manifests and the public packages whose resolved manifest/source must match.
Patches apply only in private copies and have recorded digests. The existing
host interface is `check --root ROOT` and `build --root ROOT --work WORK --out
INITRD`; `host_args` selects a deployment when supported. Other hosts need a
caller-owned adapter in their existing package.

Private manifests receive declared Cargo path patches. First preparation resolves
development locks there, retaining original/resolved lock identities. Subsequent
commands use `--locked`. There is no second complete package lock format.
Prepared inputs are verified before reuse. Failed preparation is retained;
retry uses an independent context. Tool/config/patch changes select a new context.
Ordinary execution does not update an existing baseline or a user lockfile.

Units declare their own source/workspace and baseline tests. A baseline must be
a host or target test from that Unit's source; it cannot silently select a system
scenario. Tests are explicitly host (Cargo test), target (Cargo binary plus
existing runner) or scenario (build/run a selected test deployment). Clusters
can nest and organize Units/shared crates; collaboration acceptance runs only
declared scenarios. No
scenario means an error. Repository, workspace, Cluster and Unit need not coincide.

Runner arguments are argv, not shell recipes. Substitutions: `{sources}`, `{tool}`,
`{initrd}`, `{kernel}`, `{firmware}`, `{evidence}`, `{harts}`; target-test runners
also receive `{executable}`. The runner owns framework and semantic acceptance.
Success is not inferred from QEMU exit alone. Each Hart configuration runs once,
without automatic retry. The pipe example retains its Root/probe, Kernel's
embedded-test `scene`, cargo-qemu-test 0.3.2's 60-second guest limit, documented
markers and native task accounting. `run.py` adapts that existing framework and
checks evidence; it does not reproduce pipe RPC tests. The QEMU shim removes
implicit `-nographic` using Source's existing serial mechanism and waits for
the output copier to finish.

The pipe example is a provider integration fixture. An independent product
example is `examples/source/vanllia.json`: it selects the existing Source
Integration's `product` Deployment at f82ee2a9, initializes its fixed Kernel and
System gitlinks, and calls its existing `scripts/qtest.nu` driver/judge at 4/8
Harts, with its existing terminal/session.py input driver. `source/host.patch`
adds the explicit-root interface to its existing CLI and fixes the helper
launcher's PID handoff with `exec`; assertions and guest limits are unchanged.
`dependency_alias: "image"` preserves its existing imports/tests. No second
Integration crate is created and no product list is moved into the tool. Other
product/Cluster scenarios need caller-owned configuration. QEMU is not hardware
validation.

Sources can list fixed `submodules` to initialize. Their commits come from the
parent's pinned gitlinks and are recorded. Local substitution of a source with
gitlinks is rejected: declare independently substitutable sources instead.

## Publication and evidence

Only successful Cargo invocations' reported package/bin/non-test executables
are accepted, including Cargo cache hits. ELF headers are parsed and known target
machines checked. SHA-256 snapshots remain stable across later builds. Logical
aliases still share paths. Existing closure, startup, slimming, manifest and
bootstrap capsule encoding remain the implementation.

OS file locks protect work/output paths, including canonical aliases. Lock files
remain on disk to keep one inode; close releases ownership. Final initrd bytes
are fully generated, synced in a sibling temporary file, then renamed. Failure
preserves the last successful output. Cleanup failure is reported. This is local
filesystem atomicity, not a distributed lock or power-loss durability promise.

Each CLI attempt owns separate outputs/evidence. Only complete successful
build/run attempts replace published deployment pointers. Failed runs never
start an earlier initrd and preserve the previous pointer. Different deployment,
configuration and baseline/override contexts have isolated outputs. `build`
establishes artifact production; `run` establishes declared runner acceptance.

Evidence includes source revisions/file hashes, patch hashes, original/development
lock identities, workspace Cargo metadata, Rust version, commands/status/timing,
image coordinates/snapshots, initrd/Kernel/firmware digests and runtime records.
Failed commands retain capture paths. Caches need not roll back.

The Linux CLI handles interrupt/termination, owns execution process groups and
reaps adopted children. Timeout/cancellation stops and waits; unconfirmed stops
reserve the state against reuse. Descendant leaks fail even when the leader
returns zero; cleanup failures remain separate evidence. Library embedders must
arrange cancellation (`workflow::cancel`) and child reaping themselves.

## Compatibility

Rename dependency/import to `vanllia-build`/`vanllia_build`, or explicitly use a
Cargo dependency alias. GitHub repository identity is unchanged. The known pipe
host migration is in `host.patch`; the historical Source CLI is not silently
rewritten. Public function signatures/byte formats remain compatible. Returned
paths now identify frozen snapshots rather than mutable Cargo outputs. Dynamic
coordinates are interned for process lifetime because upstream ImageBuild uses
static strings; long-running embedders should bound configurations or use CLI
processes. Historical extraction/provenance records remain unchanged.
