# Dynamic build validation

Validation uses nightly-2026-10-09, public exact Git revisions and real Cargo
locks; temporary development path patches are not the release evidence.

- CLI host tests: 38 passed, including two real RV64 build fixtures, nine discovery
  cases, ten selection cases, seven source preparation cases, two native Cargo
  overlay cases, six typed lifecycle verdict cases and two workspace cases.
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

Single-core QEMU 10.0.13 with OpenSBI 1.6 passed the minimal Pipe case, using
exact artifact hashes and System native exit/reclamation receipts. UART CRLF
translation was exposed by the first run; the serial adapter now normalizes only
that line-ending suffix and a regression rejects extra control bytes. Each run
records `verdict.json` tying the case/result to initrd, kernel and firmware hashes.
Full product runtime and SMP acceptance have not been run.
No performance improvement has been measured from removing kernel event logging.
