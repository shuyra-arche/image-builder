# Image-builder publication checkpoint

Source base: `aaed018fb2c6b9088f2d06fb6a8d5e23f984a21a`.
Public destination: `https://github.com/shuyra-arche/image-builder.git`.
Package/library: `image-builder` / `image_builder`.

Root ran host tests with explicit `x86_64-unknown-linux-gnu`, `--locked --offline`
and inherited `CARGO_NET_OFFLINE=true`. Nine pure cases and five real Cargo
artifact/packing cases passed, raw 0. Fixture artifacts target
`riscv64gc-unknown-none-elf`; real installed llvm-objcopy is on PATH. The fixture
keeps the original exact public execution/mold/env revisions and profile/feature
cases. The added case resolves two image names to one real binary and packs
both logical identities.

Cargo normally regenerated the root lock from the original snapshot after the
package rename and removal of Source dev-dependencies. Existing package tuples,
versions, revisions and checksums are unchanged; only unused dependencies are
pruned and the root package is renamed. The hand-pruned intermediate lock is not
used as publication evidence. Metadata has 21 packages and no Source path edge.

Original 25 test names are uniquely accounted for: 13 here, 12 in the Source CLI
integration target. The alias case is additional. Raw logs, original test map,
snapshot hashes and independent review are in the cloud extraction evidence.

Clean public-checkout validation and the Source full-SHA pin, removal, CLI tests,
actual image builds and fixed Kernel/System integration are separate gates after
publication. None is claimed by the provider-only host result. Final writes
remain non-atomic and callers serialize a work/output pair, as documented.
