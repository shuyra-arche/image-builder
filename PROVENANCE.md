# Source provenance

This repository starts with the image-builder implementation from
`https://github.com/anreonyr/sqware.git` at full commit
`aaed018fb2c6b9088f2d06fb6a8d5e23f984a21a` (Source PR47), under `crates/image`.
The Source history and original attribution remain authoritative and unchanged.
This is a new component snapshot, not a rewrite of that history.

`MIGRATION-MAP.json` records original file hashes, destination hashes and split
ownership. Core capsule Error/Region/Plan and BuildGroup are separated by data
structure. Inventory/order functions become explicit public planning operations
used by the same build path. The one functional correction fans a resolved Cargo
binary out to every matching logical image name, covered by a real artifact and
packing regression. Other original byte format, error and output behavior stays
at the frozen checkpoint.

Original tests map to 13 provider cases and 12 Source Integration cases. The
additional image-alias regression brings provider tests to 14. Source retains
actual composition and cross-component catalog checks; this provider has no
Source dependency, path backlink, local override or composition registry.

The Source snapshot contains no project-level license file. This extraction
does not choose, add or relicense one. Existing upstream dependency and source
notices remain as they were; no new license grant is asserted.
