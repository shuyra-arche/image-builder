# PR #1 review follow-up

Implementation: `7e686e6` (the full commit identity is in `native-proof.json`).
The native run's frozen library was compared byte-for-byte with all 33 committed
source, test, Cargo and development-documentation files; they match. This review
does not change the original protocol assertions, test driver or guest limits.

## Findings fixed

| Finding | Correction and regression evidence |
| --- | --- |
| A state directory inside the tool or local override checkout writes into the input and can recursively capture itself | Resolve existing ancestors, including symlink aliases, and reject overlap before creating files. Four CLI regressions verify this and read-only inspect. Three were observed failing on the original implementation. |
| A Unit can shadow a declared Cluster system scenario or nested Cluster | Reject conflicting names and dispatch Cluster scenarios directly to the declared Test. The name-shadow regression was observed failing before the fix. |
| Diagnostic I/O can hide `stop_confirmed=false` before the state is reserved | Preserve the original error across result/preparation/outcome recording failures; reserve before optional outcome writes. Regression tests combine these failures without leaving a live unkillable process. |
| An attempt can claim success before publication fails | Record `prepared` without a success claim; only the atomic `current.json` pointer establishes `published` success. A real rename failure produces a failed outcome. |
| Replacing an output symlink changes its lock identity | Lock the final directory entry for file publication, while continuing to resolve directory aliases for Cargo work. A held-lock regression spans the rename. |
| A changed patch can be read again after its identity was recorded | Apply frozen patch bytes from the private context. A real Git fixture changes the original patch and verifies the prepared bytes are applied. |
| A tracked input can be read through an ignored symlink ancestor | Check every path component; the regression uses a real Git index plus an ignored replacement symlink. |
| Compiler/profile environment overrides are absent from context identity | Capture only explicit compilation-control keys, include them in identity, and record Cargo version. Credential prefixes are excluded and covered by a regression. |

## Executed validation

| Command / check | Exit | Result |
| --- | --- | --- |
| `cargo test --locked --offline --target x86_64-unknown-linux-gnu` | 0 | 32 passed: 20 library cases, 3 publication, 5 original real-Cargo cases, 4 CLI isolation cases |
| `cargo clippy --locked --offline --target x86_64-unknown-linux-gnu --all-targets -- -D warnings` | 0 | passed |
| CLI `override set pipe /workspace/pipe-local-proof` | 0 | dedicated observable local replacement |
| CLI `run pipe-independent` | 0 | new 4/8 Hart execution, each original framework 1 passed, 0 failed |
| CLI `override clear pipe`, then `override check` | 0 | metadata resolves the fixed baseline again; no local checkout is restored or modified |
| Native task accounting | passed | each 6 spawned / 6 reaped, no faults, natural halt |
| Frames | passed | 4 Harts: 85→85; 8 Harts: 153→153 |
| Visible local change | passed | server marker once and public-client consumer marker twice in each guest; required new UNIT args and BUILD feature retained |
| Image identity | passed | `380b0ad63e96c7d20912175e35fbf0f4fa21b3f84d136e48bdc4a18cc3ae89a9`, identical to the previous observable local proof |
| Publication boundary | passed | attempt outcome is `prepared`, with no success claim; the atomic pointer is `published`, success=true |

Rust and llvm-objcopy came from `nightly-2026-09-04`; the existing runner retained
its 60-second guest limit. Full native JSONL remains in the managed context named
by `native-proof.json`. The committed event files are explicitly selected
Spawn/Exit/Reap/Fault/Halt projections with the original JSONL digest and length.

`before-isolation.log` and `before-shadow.log` are deliberate regression
reproductions, not accepted test results. The final passing logs are separate.
Original Source product and its host acceptance are preserved in the parent
validation report; that product was not rerun in this follow-up. No hardware or
new undeclared collaboration scenario is claimed. The PR remains unmerged.
