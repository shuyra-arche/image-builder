# Dynamic build validation

Validation uses nightly-2026-10-09, public exact Git revisions and real Cargo
locks; temporary development path patches are not the release evidence.

- CLI host tests: 37 passed, including two real RV64 build fixtures, nine discovery
  cases, ten selection cases, seven source preparation cases, two native Cargo
  overlay cases, five typed lifecycle verdict cases and two workspace cases.
- Host library: 22 inline tests passed, including native nonzero observation,
  timeout, descendant cleanup and evidence failure paths.
- A deterministic fork test reproduced inherited descriptors retaining a dropped
  parent's lock. Lease guards now explicitly unlock before closing, and the
  regression passes. The initial parallel preparation failure is not discarded.
- System's new RV64 executable and a minimal System/Pipe/Rust roundtrip image
  have been built. Runtime ELF stripping keeps debug files on the host.
- The legacy executable and four tests for its retired CLI are removed. Existing
  library publication and Cargo-artifact tests remain separate checks.
- Real fixture builds distinguish System-generated runtime resources from packed
  files and reject undeclared missing resources while preserving prior artifacts.

No QEMU or guest acceptance was run. The runner, new guest test, native receipt
delivery and full product runtime have not yet been accepted in a guest.
No performance improvement has been measured from removing kernel event logging.
