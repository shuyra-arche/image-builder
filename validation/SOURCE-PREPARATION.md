# Source preparation and local Cargo overlay checkpoint

This extends the declaration/planning checkpoint with `vanllia fetch` and
`vanllia fetch --update`. Source selectors are validated and lowered into an
explicit local-or-remote request; a pure policy function chooses local use,
pinned cache reuse, or explicit resolution. Missing Git submodules are initialized
without checking out over already initialized developer repositories.

Resolved repositories are stored by URL and commit under `.vanllia/sources`.
`vanllia.lock` records those repositories only. Preparation publishes a changed
lock only after every source succeeds. Read-only discovery locates cached locked
sources without accessing remotes. Managed source modifications are diagnosed;
path sources preserve their uncommitted files.

Cargo overlays are derived from repository origins and real package metadata.
The resolver uses Cargo 1.97+'s supported `resolver.lockfile-path` configuration
to keep Cargo-owned development lockfiles outside source repositories. The
resolved graph is inspected to reject a selected local package that Cargo leaves
bound to its original Git source. The actual build path will reuse these same
configuration and lockfile inputs.

Validation on `nightly-2026-10-09`, host `x86_64-unknown-linux-gnu`:

- All 27 CLI Rust tests passed: the original 18 plus 7 source-preparation and
  2 Cargo-overlay tests.
- New source tests use local Git repositories and exercise branch/tag/remote-ref
  resolution, locked cache reuse without a remote, explicit updates, preservation
  of the previous lock after failure, modified cache detection, selector
  ambiguity, and preservation of local source edits.
- The Cargo-overlay test actually compiles and runs a small native Rust binary.
  Its assertion distinguishes local modified dependency behavior from upstream
  behavior. Source Cargo locks remain absent while the isolated lock is produced.
- A second overlay test rejects two checkouts with the same repository/package
  identity, independently of their discovery order.
- Clippy with `-D warnings` passed after addressing its diagnostics.
- No QEMU process or guest/device acceptance was run.

Remote preparation is now implemented. Runtime Unit encoding, dynamic System
consumption, complete product build/run/test, and legacy entry-point replacement
remain in progress.
