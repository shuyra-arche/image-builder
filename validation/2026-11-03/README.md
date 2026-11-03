# Vanllia validation — 2026-11-03

These are recorded local results, not a CI or hardware attestation. The final
host-tested implementation is `f272ec3e71c3c80e7eb900d384b90f5a67408322`;
subsequent changes only add this review evidence. Native executions were collected
during implementation, before the final publication-pointer/inspector cleanup.
Each proof identifies its actual managed context; do not infer that every
historical executable came from the final implementation commit.

## Commands and results

All host commands used `RUSTUP_TOOLCHAIN=nightly-2026-09-04` and explicit
`--target x86_64-unknown-linux-gnu`. The real Cargo fixtures and deployed images
target `riscv64gc-unknown-none-elf`. QEMU was 10.0.13; Cargo jobs=4.

| Command / existing suite | Exit | Result |
| --- | --- | --- |
| `cargo test --locked --offline --target x86_64-unknown-linux-gnu` | 0 | 20 tests: original 9 pure + 5 real-Cargo cases, plus 6 publication/process cases |
| `cargo clippy --locked --offline --target x86_64-unknown-linux-gnu --all-targets -- -D warnings` | 0 | passed |
| pipe `cargo test --locked --offline --manifest-path tests/Cargo.toml --target x86_64-unknown-linux-gnu` | 0 | original 12 passed |
| Source `cargo test --locked --offline --manifest-path tools/shuyra-image/Cargo.toml --target x86_64-unknown-linux-gnu` | 0 | original 9 composition + 3 catalog passed |
| CLI `test pipe` | 0 | original host baseline through the Unit boundary |
| CLI `test source-composition` with Source configuration | 0 | existing consumer tests passed after adaptation |
| CLI `run pipe-independent` with local override, then clear and fixed build/run | 0 | both 4/8 Hart configurations passed; cleared image byte-equal to original baseline |
| CLI `cluster pipe-unit` | 0 | its declared existing integration passed at 4/8 Harts |
| CLI `run product` with Source configuration | 0 | original qtest and terminal/session.py passed at 4/8 Harts |
| Unit baseline incorrectly selecting a scenario / Cluster cycle | 2, expected | rejected before preparation |
| SIGINT during fixed-source preparation | 2, expected | owned process group stopped; no publication |
| CLI `inspect` | 0 | no new contexts or source preparation |

CLI commands use this prefix, substituting the configuration/state as needed:

```sh
cargo run --locked --target x86_64-unknown-linux-gnu -- \
  --config examples/pipe/vanllia.json --state /tmp/vanllia
```

The six additional tests cover real mapping/Cargo/packing failure preservation,
snapshot stability after a later Cargo rebuild, conflicting work/output ownership,
timeout, leaked descendants and evidence-write failure. Original assertions,
fixture content, guest limits and capsule encoding are retained.

## Runtime evidence

| Deployment | Harts | Original framework | Frames | Native spawn/reap |
| --- | --- | --- | --- | --- |
| pipe baseline | 4 / 8 | each 1 passed, 0 failed | 85→85 / 153→153 | each 6/6 |
| pipe local replacement | 4 / 8 | each 1 passed, 0 failed | 85→85 / 153→153 | each 6/6 |
| Source product | 4 / 8 | each 1 passed, 0 failed | 85→85 / 153→153 | each 14/14 |

`runs/` retains original framework/console records and the Spawn/Exit/Reap/Fault/
Halt projection of native JSONL evidence. `proofs/` records full native-file
hashes, image identities and actual reported test-Kernel executable identities.
The projection is explicitly derived; full native JSONL and binary artifacts
remain in the managed delivery bundle at `/workspace/vanllia-delivery`, not in Git.
`SHA256SUMS` covers all files in this evidence directory except itself.
Captured logs are byte-preserved, including framework trailing newlines; their
whitespace is data rather than source formatting. The local attributes preserve
these evidence bytes and exempt only log files from whitespace style checks.

The local proof changes UNIT arguments, BUILD features and public API/client
markers in a dedicated checkout. The server requires the new feature to compile
and checks the new UNIT arguments at runtime; the consumers call the new client
API. The changed ELF bytes are present in the initrd. All participating main,
host-test and integration workspace package resolutions were checked. The
original pipe checkout stayed clean, and the dedicated checkout's diff stayed
unchanged throughout. The proof diff is included; it is not a product change.

The pipe integration retains parent/peer semantics, named zero exits, Root
SHUTDOWN_OK, native task accounting and System-controlled retirement of the
resident pipe. The resident's numeric exit remains unobserved, as in the original
mechanism. Source product retains its existing login/session assertions, named
`system: done` zero exit, natural halt, frame equality and no native fault/panic.

## Fixed input identities

- pipe: `4fce76372beb6c5ab8567232516f156b3d1032eb`.
- Source: `f82ee2a9751b09563ca0f21577c212ab4470c8e4`.
- Kernel: `3aa0a155f825fbf377fc8d04009472ca67cbbf8c`.
- Source's System gitlink: `5a792ff2d39901a970925b1a6d63206adf6c16d8`.
- Firmware: Source's fixed SBI.bin, 277880 bytes, SHA-256
  `c53d58aa9ab17f95fa1af159f0bf05fad5c5622ebc0f1084463435fa6e5752b6`.
- Baseline pipe initrd:
  `269306c40d927716422af783a572bfbaf5d7e9046bac4eb7782dde15e5692eda`.
- Local pipe initrd:
  `380b0ad63e96c7d20912175e35fbf0f4fa21b3f84d136e48bdc4a18cc3ae89a9`.
- Source product initrd:
  `763ef957669b206f17f5c9a2772ebf66e26d7e8d3c644161e9415dd0f936918b`.

Early runs used the existing LLVM 23.1.3 objcopy. The final pinned-toolchain
LLVM 23.1.1 objcopy rebuilt byte-identical product bytes and the host suite passed
again. `proofs/pinned-llvm.json` records this distinction and tool digest.

## Failures, corrections and limits

Failures were not erased or automatically retried. Separate attempts follow
identified adapter/configuration fixes:

- An initially wrong Kernel bin coordinate was rejected by Cargo; the actual bin
  is `shuyra`. Nothing was published from that attempt.
- A simple `exit` input adapter reached product login and hit the unchanged
  60-second guest timeout. It was replaced with the existing terminal/session.py
  driver, without copying or changing its assertions.
- The existing Source helper launcher recorded a shell PID and leaked its Python
  child. The consumer patch adds `exec` at that handoff; framework success alone
  did not count while cleanup failed. A bounded drain waits for already-stopping
  helpers; live descendants still fail and are stopped before returning.
- Source's QEMU shim changes cwd to the Source root. Its native exports are now
  archived from that actual directory separately for each Hart configuration.
- Early tee/diagnostic commands left exited processes owned by container PID 1;
  the current process cannot reap those historical zombies. The final CLI uses
  subreaping, and accepted final executions leave no active QEMU/helper process.

Additional target-Unit baselines, undeclared multi-level Cluster product scenarios
and physical hardware have not been separately accepted. No substitute scenario
was invented for them. Library embedders must arrange cancellation/reaping;
the Linux CLI installs these mechanisms. See [DEVELOPMENT.md](../../DEVELOPMENT.md)
for the supported development workflow and compatibility boundaries.
